#![no_std]

extern crate alloc;

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Write;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use abi::errors::Errno;
use abi::ids::HandleId;
use abi::types::{Edge, GraphProp};
use abi::PredicateId;
use llm::{ChatDelta, ChatRequest, ChatStream, FinishReason, LlmError, Message, Role, StreamingLlmClient};
use stem::thing::ThingId;

const DEFAULT_PROMPT_ID: &str = "describe:v1";
const DEFAULT_MODEL_ID: &str = "stub:something";
const DEFAULT_SYSTEM_PROMPT: &str = "You are a description engine. Describe the Thing based on the provided graph view.";
const DEFAULT_MAX_DESCRIPTION_BYTES: usize = 16 * 1024;
const DEFAULT_MAX_PROMPT_BYTES: usize = 4 * 1024;
const DEFAULT_MAX_PROMPT_PROPS: usize = 64;
const DEFAULT_MAX_PROMPT_EDGES: usize = 64;
const DEFAULT_CACHE_EDGE_SCAN: usize = 128;

const KIND_DESCRIPTION: &str = "description";
const REL_HAS_DESCRIPTION: &str = "has_description";
const KEY_DESCRIPTION_FOR: &str = "description.for";
const KEY_DESCRIPTION_MODE: &str = "description.mode";
const KEY_DESCRIPTION_TEXT: &str = "description.text";
const KEY_DESCRIPTION_INPUT_HASH: &str = "description.input_hash";
const KEY_DESCRIPTION_PROMPT_ID: &str = "description.prompt_id";
const KEY_DESCRIPTION_MODEL_ID: &str = "description.model_id";
const KEY_DESCRIPTION_CREATED_AT: &str = "description.created_at_monotonic";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DescribeMode {
    Short,
    Long,
}

impl DescribeMode {
    fn as_u64(self) -> u64 {
        match self {
            DescribeMode::Short => 0,
            DescribeMode::Long => 1,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            DescribeMode::Short => "short",
            DescribeMode::Long => "long",
        }
    }
}

#[derive(Clone, Debug)]
pub enum ViewSpec {
    NodeOnly,
    Neighborhood {
        max_hops: u8,
        max_edges: u32,
        edge_whitelist: Vec<PredicateId>,
    },
}

#[derive(Clone, Debug)]
pub struct DescribeRequest {
    pub thing_id: ThingId,
    pub view_spec: ViewSpec,
    pub mode: DescribeMode,
}

#[derive(Clone, Debug)]
pub struct DescriptionMeta {
    pub model_id: String,
    pub prompt_id: String,
    pub input_hash: [u8; 32],
    pub created_at_monotonic: u64,
}

#[derive(Clone, Debug)]
pub struct DescribeResponse {
    pub description_id: ThingId,
    pub text: String,
    pub meta: DescriptionMeta,
}

#[derive(Clone, Debug)]
pub struct PromptPacket {
    pub text: String,
    pub input_hash: [u8; 32],
}

#[derive(Clone, Debug)]
pub struct DescriptionConfig {
    pub model_id: &'static str,
    pub prompt_id: &'static str,
    pub system_prompt: &'static str,
    pub max_description_bytes: usize,
    pub max_prompt_bytes: usize,
    pub max_prompt_props: usize,
    pub max_prompt_edges: usize,
    pub cache_edge_scan: usize,
}

impl Default for DescriptionConfig {
    fn default() -> Self {
        Self {
            model_id: DEFAULT_MODEL_ID,
            prompt_id: DEFAULT_PROMPT_ID,
            system_prompt: DEFAULT_SYSTEM_PROMPT,
            max_description_bytes: DEFAULT_MAX_DESCRIPTION_BYTES,
            max_prompt_bytes: DEFAULT_MAX_PROMPT_BYTES,
            max_prompt_props: DEFAULT_MAX_PROMPT_PROPS,
            max_prompt_edges: DEFAULT_MAX_PROMPT_EDGES,
            cache_edge_scan: DEFAULT_CACHE_EDGE_SCAN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DescribeError {
    Graph(Errno),
    Llm(LlmError),
    NotFound,
}

impl From<Errno> for DescribeError {
    fn from(err: Errno) -> Self {
        DescribeError::Graph(err)
    }
}

impl From<LlmError> for DescribeError {
    fn from(err: LlmError) -> Self {
        DescribeError::Llm(err)
    }
}

pub trait DescribeSink {
    fn on_delta(&mut self, delta: &ChatDelta);
}

impl<F> DescribeSink for F
where
    F: FnMut(&ChatDelta),
{
    fn on_delta(&mut self, delta: &ChatDelta) {
        self(delta);
    }
}

pub trait Clock {
    fn monotonic_ns(&self) -> u64;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn monotonic_ns(&self) -> u64 {
        stem::time::monotonic_ns()
    }
}

pub trait DescribeGraph {
    fn intern(&mut self, s: &str) -> Result<u64, Errno>;
    fn get_kind(&self, id: ThingId) -> Option<u64>;
    fn get_props(&self, id: ThingId, out: &mut [GraphProp]) -> Result<usize, Errno>;
    fn get_edges(&self, id: ThingId, out: &mut [Edge]) -> Result<usize, Errno>;
    fn prop_get(&self, id: ThingId, key: &str) -> Result<u64, Errno>;
    fn prop_set(&mut self, id: ThingId, key: &str, value: u64) -> Result<(), Errno>;
    fn create_node(&mut self, kind: &str) -> Result<ThingId, Errno>;
    fn link(&mut self, src: ThingId, rel: &str, dst: ThingId) -> Result<(), Errno>;
    fn bytespace_create(&mut self, len: usize) -> Result<ThingId, Errno>;
    fn bytespace_write(&mut self, id: ThingId, offset: usize, data: &[u8]) -> Result<usize, Errno>;
    fn bytespace_info(&self, id: ThingId) -> Result<usize, Errno>;
    fn bytespace_read(&self, id: ThingId, offset: usize, out: &mut [u8]) -> Result<usize, Errno>;

    fn read_bytespace(&self, id: ThingId) -> Result<Vec<u8>, Errno> {
        let size = self.bytespace_info(id)?;
        if size == 0 {
            return Ok(Vec::new());
        }
        let mut out = Vec::with_capacity(size);
        out.resize(size, 0);
        let mut offset = 0usize;
        while offset < size {
            let end = core::cmp::min(offset + 4096, size);
            let read = self.bytespace_read(id, offset, &mut out[offset..end])?;
            if read == 0 {
                break;
            }
            offset = offset.saturating_add(read);
        }
        Ok(out)
    }
}

pub struct SysGraph;

impl DescribeGraph for SysGraph {
    fn intern(&mut self, s: &str) -> Result<u64, Errno> {
        stem::thing::sys::intern(s).map(|v| v as u64)
    }

    fn get_kind(&self, id: ThingId) -> Option<u64> {
        stem::thing::sys::get_kind(id).ok().map(|k| k.0)
    }

    fn get_props(&self, id: ThingId, out: &mut [GraphProp]) -> Result<usize, Errno> {
        stem::thing::sys::get_props(id, out)
    }

    fn get_edges(&self, id: ThingId, out: &mut [Edge]) -> Result<usize, Errno> {
        stem::thing::sys::get_edges(id, out)
    }

    fn prop_get(&self, id: ThingId, key: &str) -> Result<u64, Errno> {
        stem::thing::sys::prop_get(id, key)
    }

    fn prop_set(&mut self, id: ThingId, key: &str, value: u64) -> Result<(), Errno> {
        stem::thing::sys::prop_set(id, key, value)
    }

    fn create_node(&mut self, kind: &str) -> Result<ThingId, Errno> {
        stem::thing::sys::create_node(kind)
    }

    fn link(&mut self, src: ThingId, rel: &str, dst: ThingId) -> Result<(), Errno> {
        stem::thing::sys::link(src, rel, dst)
    }

    fn bytespace_create(&mut self, len: usize) -> Result<ThingId, Errno> {
        stem::thing::sys::bytespace_create(len, 0, 0)
    }

    fn bytespace_write(&mut self, id: ThingId, offset: usize, data: &[u8]) -> Result<usize, Errno> {
        stem::thing::sys::bytespace_write(id, offset, data)
    }

    fn bytespace_info(&self, id: ThingId) -> Result<usize, Errno> {
        stem::thing::sys::bytespace_info(id)
    }

    fn bytespace_read(&self, id: ThingId, offset: usize, out: &mut [u8]) -> Result<usize, Errno> {
        stem::thing::sys::bytespace_read(id, offset, out)
    }
}

pub struct DescriptionService<'a, C: StreamingLlmClient, Clk: Clock> {
    client: &'a C,
    clock: &'a Clk,
    config: DescriptionConfig,
}

impl<'a, C: StreamingLlmClient, Clk: Clock> DescriptionService<'a, C, Clk> {
    pub fn new(client: &'a C, clock: &'a Clk) -> Self {
        Self {
            client,
            clock,
            config: DescriptionConfig::default(),
        }
    }

    pub fn with_config(client: &'a C, clock: &'a Clk, config: DescriptionConfig) -> Self {
        Self { client, clock, config }
    }

    pub fn describe<G: DescribeGraph>(
        &self,
        graph: &mut G,
        req: DescribeRequest,
        sink: Option<&mut dyn DescribeSink>,
    ) -> Result<DescribeResponse, DescribeError> {
        let prompt = build_prompt_packet(graph, req.thing_id, &req.view_spec, &self.config)?;
        let input_hash_hex = hex_encode(&prompt.input_hash);

        if let Some(cached) = find_cached_description(
            graph,
            req.thing_id,
            req.mode,
            self.config.prompt_id,
            &input_hash_hex,
            &prompt.input_hash,
            self.config.model_id,
            self.config.cache_edge_scan,
        )? {
            return Ok(cached);
        }

        let chat_req = ChatRequest {
            system: Some(self.config.system_prompt.to_string()),
            messages: vec![Message {
                role: Role::User,
                content: prompt.text.clone(),
            }],
            temperature: None,
            max_tokens: match req.mode {
                DescribeMode::Short => Some(128),
                DescribeMode::Long => Some(512),
            },
            stop: Vec::new(),
            metadata: BTreeMap::new(),
        };

        let mut stream = self.client.chat_stream(chat_req)?;
        let (text, _finish) = drain_stream(
            &mut *stream,
            self.config.max_description_bytes,
            sink,
        )?;

        let created_at = self.clock.monotonic_ns();
        let description_id = persist_description(
            graph,
            req.thing_id,
            req.mode,
            &text,
            self.config.model_id,
            self.config.prompt_id,
            &input_hash_hex,
            created_at,
        )?;

        Ok(DescribeResponse {
            description_id,
            text,
            meta: DescriptionMeta {
                model_id: self.config.model_id.to_string(),
                prompt_id: self.config.prompt_id.to_string(),
                input_hash: prompt.input_hash,
                created_at_monotonic: created_at,
            },
        })
    }
}

pub fn build_prompt_packet<G: DescribeGraph>(
    graph: &mut G,
    thing_id: ThingId,
    view: &ViewSpec,
    config: &DescriptionConfig,
) -> Result<PromptPacket, DescribeError> {
    let mut text = String::new();
    let skip_rel = graph.intern(REL_HAS_DESCRIPTION).ok();

    let _ = writeln!(text, "thing_id {}", thing_id.to_u64_lossy());

    if let Some(kind) = graph.get_kind(thing_id) {
        let _ = writeln!(text, "kind {}", kind);
    }

    match view {
        ViewSpec::NodeOnly => {
            let _ = writeln!(text, "view node_only");
            append_node_only(graph, thing_id, &mut text, config, skip_rel)?;
        }
        ViewSpec::Neighborhood {
            max_hops,
            max_edges,
            edge_whitelist,
        } => {
            let _ = writeln!(text, "view neighborhood hops={} max_edges={}", max_hops, max_edges);
            append_neighborhood(
                graph,
                thing_id,
                *max_hops,
                *max_edges as usize,
                edge_whitelist,
                &mut text,
                config,
                skip_rel,
            )?;
        }
    }

    truncate_utf8(&mut text, config.max_prompt_bytes);

    let hash = blake3::hash(text.as_bytes());
    Ok(PromptPacket {
        text,
        input_hash: *hash.as_bytes(),
    })
}

fn append_node_only<G: DescribeGraph>(
    graph: &G,
    thing_id: ThingId,
    text: &mut String,
    config: &DescriptionConfig,
    skip_rel: Option<u64>,
) -> Result<(), DescribeError> {
    let mut props = vec![GraphProp::default(); config.max_prompt_props];
    let prop_count = graph.get_props(thing_id, &mut props)?;
    props.truncate(prop_count);
    props.sort_by_key(|p| (p.key, p.value));

    let _ = writeln!(text, "props {}", prop_count);
    for prop in &props {
        if text.len() >= config.max_prompt_bytes {
            break;
        }
        let _ = writeln!(text, "prop {} {}", prop.key, prop.value);
    }

    let mut edges = vec![Edge::default(); config.max_prompt_edges];
    let edge_count = graph.get_edges(thing_id, &mut edges)?;
    edges.truncate(edge_count);

    if let Some(skip) = skip_rel {
        edges.retain(|e| e.predicate.to_u64_lossy() != skip);
    }

    edges.sort_by_key(|e| (e.predicate.to_u64_lossy(), e.to.to_u64_lossy()));

    let _ = writeln!(text, "edges {}", edges.len());
    for edge in &edges {
        if text.len() >= config.max_prompt_bytes {
            break;
        }
        let _ = writeln!(
            text,
            "edge {} {}",
            edge.predicate.to_u64_lossy(),
            edge.to.to_u64_lossy()
        );
    }

    Ok(())
}

fn append_neighborhood<G: DescribeGraph>(
    graph: &G,
    root: ThingId,
    max_hops: u8,
    max_edges: usize,
    whitelist: &[PredicateId],
    text: &mut String,
    config: &DescriptionConfig,
    skip_rel: Option<u64>,
) -> Result<(), DescribeError> {
    let mut visited: BTreeSet<ThingId> = BTreeSet::new();
    let mut frontier: Vec<(ThingId, u8)> = Vec::new();
    let mut edges: Vec<(ThingId, ThingId, ThingId)> = Vec::new();
    let mut nodes: Vec<ThingId> = Vec::new();

    visited.insert(root);
    frontier.push((root, 0));

    while let Some((node, depth)) = frontier.pop() {
        nodes.push(node);
        if depth >= max_hops {
            continue;
        }

        let mut edge_buf = vec![Edge::default(); config.max_prompt_edges];
        let count = graph.get_edges(node, &mut edge_buf)?;
        for edge in edge_buf.iter().take(count) {
            if edges.len() >= max_edges {
                break;
            }
            if let Some(skip) = skip_rel {
                if edge.predicate.to_u64_lossy() == skip {
                    continue;
                }
            }
            if !whitelist.is_empty() && !predicate_whitelisted(edge.predicate, whitelist) {
                continue;
            }
            edges.push((edge.from, edge.predicate, edge.to));
            if visited.insert(edge.to) {
                frontier.push((edge.to, depth.saturating_add(1)));
            }
        }
    }

    nodes.sort_by_key(|id| id.to_u64_lossy());
    edges.sort_by_key(|(from, pred, to)| {
        (from.to_u64_lossy(), pred.to_u64_lossy(), to.to_u64_lossy())
    });

    let _ = writeln!(text, "nodes {}", nodes.len());
    for id in &nodes {
        if text.len() >= config.max_prompt_bytes {
            break;
        }
        if let Some(kind) = graph.get_kind(*id) {
            let _ = writeln!(text, "node {} kind {}", id.to_u64_lossy(), kind);
        } else {
            let _ = writeln!(text, "node {}", id.to_u64_lossy());
        }
    }

    let _ = writeln!(text, "edges {}", edges.len());
    for (from, pred, to) in &edges {
        if text.len() >= config.max_prompt_bytes {
            break;
        }
        let _ = writeln!(
            text,
            "edge {} {} {}",
            from.to_u64_lossy(),
            pred.to_u64_lossy(),
            to.to_u64_lossy()
        );
    }

    Ok(())
}

fn predicate_whitelisted(pred: ThingId, whitelist: &[PredicateId]) -> bool {
    whitelist.iter().any(|allowed| allowed.0 == pred.0)
}

fn find_cached_description<G: DescribeGraph>(
    graph: &mut G,
    thing_id: ThingId,
    mode: DescribeMode,
    prompt_id: &str,
    input_hash_hex: &str,
    input_hash: &[u8; 32],
    model_id_fallback: &str,
    cache_edge_scan: usize,
) -> Result<Option<DescribeResponse>, DescribeError> {
    let rel_id = graph.intern(REL_HAS_DESCRIPTION)?;
    let mut edges = vec![Edge::default(); cache_edge_scan];
    let edge_count = graph.get_edges(thing_id, &mut edges)?;
    edges.truncate(edge_count);

    for edge in edges {
        if edge.predicate.to_u64_lossy() != rel_id {
            continue;
        }
        let desc_id = edge.to;
        let existing_mode = graph.prop_get(desc_id, KEY_DESCRIPTION_MODE).unwrap_or(0);
        if existing_mode != mode.as_u64() {
            continue;
        }
        let existing_prompt = read_string_prop(graph, desc_id, KEY_DESCRIPTION_PROMPT_ID)?;
        if existing_prompt.as_deref() != Some(prompt_id) {
            continue;
        }
        let existing_hash = read_string_prop(graph, desc_id, KEY_DESCRIPTION_INPUT_HASH)?;
        if existing_hash.as_deref() != Some(input_hash_hex) {
            continue;
        }
        let text = read_string_prop(graph, desc_id, KEY_DESCRIPTION_TEXT)?
            .unwrap_or_else(|| String::new());
        let model_id = read_string_prop(graph, desc_id, KEY_DESCRIPTION_MODEL_ID)?
            .unwrap_or_else(|| model_id_fallback.to_string());
        let created_at = graph
            .prop_get(desc_id, KEY_DESCRIPTION_CREATED_AT)
            .unwrap_or(0);
        return Ok(Some(DescribeResponse {
            description_id: desc_id,
            text,
            meta: DescriptionMeta {
                model_id,
                prompt_id: prompt_id.to_string(),
                input_hash: *input_hash,
                created_at_monotonic: created_at,
            },
        }));
    }

    Ok(None)
}

fn persist_description<G: DescribeGraph>(
    graph: &mut G,
    thing_id: ThingId,
    mode: DescribeMode,
    text: &str,
    model_id: &str,
    prompt_id: &str,
    input_hash_hex: &str,
    created_at: u64,
) -> Result<ThingId, DescribeError> {
    let desc_id = graph.create_node(KIND_DESCRIPTION)?;
    graph.prop_set(desc_id, KEY_DESCRIPTION_FOR, thing_id.to_u64_lossy())?;
    graph.prop_set(desc_id, KEY_DESCRIPTION_MODE, mode.as_u64())?;
    graph.prop_set(desc_id, KEY_DESCRIPTION_CREATED_AT, created_at)?;
    set_string_prop(graph, desc_id, KEY_DESCRIPTION_TEXT, text)?;
    set_string_prop(graph, desc_id, KEY_DESCRIPTION_INPUT_HASH, input_hash_hex)?;
    set_string_prop(graph, desc_id, KEY_DESCRIPTION_PROMPT_ID, prompt_id)?;
    set_string_prop(graph, desc_id, KEY_DESCRIPTION_MODEL_ID, model_id)?;
    graph.link(thing_id, REL_HAS_DESCRIPTION, desc_id)?;
    Ok(desc_id)
}

fn drain_stream(
    stream: &mut dyn ChatStream,
    max_bytes: usize,
    mut sink: Option<&mut dyn DescribeSink>,
) -> Result<(String, Option<FinishReason>), DescribeError> {
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    let mut out = String::new();
    let mut finish = None;

    loop {
        match stream.poll_next(&mut cx) {
            Poll::Ready(Ok(Some(delta))) => {
                if let Some(reason) = delta.finish {
                    finish = Some(reason);
                }

                let remaining = max_bytes.saturating_sub(out.len());
                if remaining == 0 {
                    finish = Some(FinishReason::Length);
                    break;
                }
                let accepted = utf8_prefix(&delta.text, remaining);
                if !accepted.is_empty() {
                    out.push_str(accepted);
                    if let Some(ref mut sink) = sink {
                        if accepted.len() == delta.text.len() {
                            sink.on_delta(&delta);
                        } else {
                            sink.on_delta(&ChatDelta {
                                text: accepted.to_string(),
                                finish: delta.finish,
                            });
                        }
                    }
                }
                if accepted.len() < delta.text.len() || out.len() >= max_bytes {
                    finish = Some(FinishReason::Length);
                    break;
                }
            }
            Poll::Ready(Ok(None)) => break,
            Poll::Ready(Err(err)) => return Err(DescribeError::Llm(err)),
            Poll::Pending => {
                #[cfg(not(test))]
                stem::thread::yield_now();
            }
        }
    }

    Ok((out, finish))
}

fn read_string_prop<G: DescribeGraph>(
    graph: &G,
    id: ThingId,
    key: &str,
) -> Result<Option<String>, Errno> {
    let bs = match graph.prop_get(id, key) {
        Ok(val) => val,
        Err(_) => return Ok(None),
    };
    if bs == 0 {
        return Ok(None);
    }
    let bytes = graph.read_bytespace(ThingId::from_u64(bs))?;
    let s = core::str::from_utf8(&bytes).ok().map(|v| v.trim_end_matches('\0'));
    Ok(s.map(|v| v.to_string()))
}

fn set_string_prop<G: DescribeGraph>(
    graph: &mut G,
    id: ThingId,
    key: &str,
    value: &str,
) -> Result<(), Errno> {
    if value.is_empty() {
        graph.prop_set(id, key, 0)?;
        return Ok(());
    }
    let bs = graph.bytespace_create(value.len())?;
    let _ = graph.bytespace_write(bs, 0, value.as_bytes())?;
    graph.prop_set(id, key, bs.to_u64_lossy())?;
    Ok(())
}

fn truncate_utf8(s: &mut String, max: usize) {
    if s.len() <= max {
        return;
    }
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    s.truncate(end);
}

fn utf8_prefix(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

fn noop_waker() -> Waker {
    unsafe fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(core::ptr::null(), &VTABLE)
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    unsafe { Waker::from_raw(RawWaker::new(core::ptr::null(), &VTABLE)) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use core::cell::Cell;
    use llm_stub::StubLlmClient;

    struct FixedClock {
        now: u64,
    }

    impl Clock for FixedClock {
        fn monotonic_ns(&self) -> u64 {
            self.now
        }
    }

    #[derive(Default)]
    struct RecordingSink {
        chunks: Vec<String>,
    }

    impl DescribeSink for RecordingSink {
        fn on_delta(&mut self, delta: &ChatDelta) {
            self.chunks.push(delta.text.clone());
        }
    }

    struct CountingClient {
        inner: StubLlmClient,
        calls: Cell<u32>,
    }

    impl CountingClient {
        fn new() -> Self {
            Self {
                inner: StubLlmClient::new(),
                calls: Cell::new(0),
            }
        }

        fn calls(&self) -> u32 {
            self.calls.get()
        }
    }

    impl StreamingLlmClient for CountingClient {
        fn chat_stream(&self, req: ChatRequest) -> Result<Box<dyn ChatStream + Send>, LlmError> {
            self.calls.set(self.calls.get().saturating_add(1));
            self.inner.chat_stream(req)
        }
    }

    #[derive(Clone)]
    struct TestNode {
        kind: u64,
        props: BTreeMap<String, u64>,
        edges: Vec<(ThingId, ThingId)>,
    }

    struct TestGraph {
        next_id: u64,
        next_symbol: u32,
        symbols: BTreeMap<String, u32>,
        nodes: BTreeMap<ThingId, TestNode>,
        bytespaces: BTreeMap<ThingId, Vec<u8>>,
    }

    impl TestGraph {
        fn new() -> Self {
            Self {
                next_id: 1,
                next_symbol: 1,
                symbols: BTreeMap::new(),
                nodes: BTreeMap::new(),
                bytespaces: BTreeMap::new(),
            }
        }

        fn alloc_id(&mut self) -> ThingId {
            let id = ThingId::from_u64(self.next_id);
            self.next_id = self.next_id.saturating_add(1);
            id
        }

        fn intern_symbol(&mut self, s: &str) -> u32 {
            if let Some(id) = self.symbols.get(s) {
                return *id;
            }
            let id = self.next_symbol;
            self.next_symbol = self.next_symbol.saturating_add(1);
            self.symbols.insert(s.to_string(), id);
            id
        }
    }

    impl DescribeGraph for TestGraph {
        fn intern(&mut self, s: &str) -> Result<u64, Errno> {
            Ok(self.intern_symbol(s) as u64)
        }

        fn get_kind(&self, id: ThingId) -> Option<u64> {
            self.nodes.get(&id).map(|node| node.kind)
        }

        fn get_props(&self, id: ThingId, out: &mut [GraphProp]) -> Result<usize, Errno> {
            let node = self.nodes.get(&id).ok_or(Errno::ENOENT)?;
            let mut idx = 0usize;
            for (key, value) in node.props.iter() {
                if idx >= out.len() {
                    break;
                }
                let key_id = self.symbols.get(key).copied().unwrap_or(0);
                out[idx] = GraphProp {
                    key: key_id,
                    _pad: 0,
                    value: *value,
                };
                idx += 1;
            }
            Ok(idx)
        }

        fn get_edges(&self, id: ThingId, out: &mut [Edge]) -> Result<usize, Errno> {
            let node = self.nodes.get(&id).ok_or(Errno::ENOENT)?;
            let mut idx = 0usize;
            for (pred, to) in node.edges.iter() {
                if idx >= out.len() {
                    break;
                }
                out[idx] = Edge {
                    from: id,
                    predicate: *pred,
                    to: *to,
                    flags: 0,
                };
                idx += 1;
            }
            Ok(idx)
        }

        fn prop_get(&self, id: ThingId, key: &str) -> Result<u64, Errno> {
            let node = self.nodes.get(&id).ok_or(Errno::ENOENT)?;
            Ok(*node.props.get(key).unwrap_or(&0))
        }

        fn prop_set(&mut self, id: ThingId, key: &str, value: u64) -> Result<(), Errno> {
            self.intern_symbol(key);
            let node = self.nodes.get_mut(&id).ok_or(Errno::ENOENT)?;
            node.props.insert(key.to_string(), value);
            Ok(())
        }

        fn create_node(&mut self, kind: &str) -> Result<ThingId, Errno> {
            let id = self.alloc_id();
            let kind_id = self.intern_symbol(kind) as u64;
            self.nodes.insert(
                id,
                TestNode {
                    kind: kind_id,
                    props: BTreeMap::new(),
                    edges: Vec::new(),
                },
            );
            Ok(id)
        }

        fn link(&mut self, src: ThingId, rel: &str, dst: ThingId) -> Result<(), Errno> {
            let pred_id = ThingId::from_u64(self.intern_symbol(rel) as u64);
            let node = self.nodes.get_mut(&src).ok_or(Errno::ENOENT)?;
            node.edges.push((pred_id, dst));
            Ok(())
        }

        fn bytespace_create(&mut self, len: usize) -> Result<ThingId, Errno> {
            let id = self.alloc_id();
            self.bytespaces.insert(id, vec![0u8; len]);
            Ok(id)
        }

        fn bytespace_write(&mut self, id: ThingId, offset: usize, data: &[u8]) -> Result<usize, Errno> {
            let buf = self.bytespaces.get_mut(&id).ok_or(Errno::ENOENT)?;
            let end = core::cmp::min(offset + data.len(), buf.len());
            let count = end.saturating_sub(offset);
            buf[offset..end].copy_from_slice(&data[..count]);
            Ok(count)
        }

        fn bytespace_info(&self, id: ThingId) -> Result<usize, Errno> {
            Ok(self.bytespaces.get(&id).map(|v| v.len()).unwrap_or(0))
        }

        fn bytespace_read(&self, id: ThingId, offset: usize, out: &mut [u8]) -> Result<usize, Errno> {
            let buf = self.bytespaces.get(&id).ok_or(Errno::ENOENT)?;
            if offset >= buf.len() {
                return Ok(0);
            }
            let end = core::cmp::min(offset + out.len(), buf.len());
            let count = end.saturating_sub(offset);
            out[..count].copy_from_slice(&buf[offset..end]);
            Ok(count)
        }
    }

    #[test]
    fn describe_streams_and_stores() {
        let mut graph = TestGraph::new();
        let thing_id = graph.create_node("test.thing").unwrap();
        graph.prop_set(thing_id, "title", 42).unwrap();

        let client = CountingClient::new();
        let clock = FixedClock { now: 1234 };
        let service = DescriptionService::new(&client, &clock);
        let mut sink = RecordingSink::default();

        let req = DescribeRequest {
            thing_id,
            view_spec: ViewSpec::NodeOnly,
            mode: DescribeMode::Short,
        };
        let resp = service.describe(&mut graph, req, Some(&mut sink)).unwrap();

        assert_eq!(sink.chunks, ["Som", "eth", "ing"]);
        assert_eq!(resp.text, "Something");

        let stored = read_string_prop(&graph, resp.description_id, KEY_DESCRIPTION_TEXT)
            .unwrap()
            .unwrap();
        assert_eq!(stored, "Something");
        assert_eq!(client.calls(), 1);
    }

    #[test]
    fn caching_skips_second_run() {
        let mut graph = TestGraph::new();
        let thing_id = graph.create_node("test.thing").unwrap();

        let client = CountingClient::new();
        let clock = FixedClock { now: 1234 };
        let service = DescriptionService::new(&client, &clock);

        let req = DescribeRequest {
            thing_id,
            view_spec: ViewSpec::NodeOnly,
            mode: DescribeMode::Short,
        };

        let mut sink1 = RecordingSink::default();
        let resp1 = service.describe(&mut graph, req.clone(), Some(&mut sink1)).unwrap();
        assert_eq!(sink1.chunks.len(), 3);

        let mut sink2 = RecordingSink::default();
        let resp2 = service.describe(&mut graph, req, Some(&mut sink2)).unwrap();

        assert_eq!(sink2.chunks.len(), 0);
        assert_eq!(resp1.description_id, resp2.description_id);
        assert_eq!(client.calls(), 1);
    }
}

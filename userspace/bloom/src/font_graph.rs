extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::vec;
use abi::ids::HandleId;
use abi::schema::{kinds, keys, rels};
use crate::asset::{AssetBank, FontAsset};
use spin::Mutex;
use stem::thing::ThingId;
use stem::thing::sys::{bytespace_info, bytespace_read, find, get_edges, intern, prop_get};

#[derive(Clone, Copy, Debug)]
pub struct FontStyle {
    pub weight: u16,
    pub width: u16,
    pub slope: u8,
}

impl Default for FontStyle {
    fn default() -> Self {
        Self { weight: 400, width: 5, slope: 0 }
    }
}

#[derive(Clone, Debug)]
pub struct ResolvedFace {
    pub face_id: ThingId,
    pub family_id: ThingId,
    pub family_name: Arc<str>,
    pub style_name: Arc<str>,
    pub file_id: ThingId,
}

#[derive(Clone, Debug)]
struct FontFamily {
    name: Arc<str>,
    faces: Vec<ThingId>,
    superfamily_id: Option<ThingId>,
}

#[derive(Clone, Debug)]
struct FontFace {
    family_id: ThingId,
    file_id: ThingId,
    style: Arc<str>,
    weight: u16,
    width: u16,
    slope: u8,
    coverage: Vec<(u32, u32)>,
}

#[derive(Clone, Debug)]
struct FontFile {
    bytespace_id: ThingId,
    size_bytes: usize,
    name: Arc<str>,
}

#[derive(Clone, Debug)]
struct FontSuperfamily {
    name: Arc<str>,
    families: Vec<ThingId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ResolveKey {
    stack_hash: u64,
    weight: u16,
    width: u16,
    slope: u8,
    codepoint: u32,
}

impl ResolveKey {
    fn new(stack_hash: u64, style: FontStyle, codepoint: u32) -> Self {
        Self {
            stack_hash,
            weight: style.weight,
            width: style.width,
            slope: style.slope,
            codepoint,
        }
    }
}

struct FontSymbols {
    contains: u64,
    covers: u64,
}

impl FontSymbols {
    fn intern() -> Self {
        Self {
            contains: intern(rels::FONT_CONTAINS).unwrap_or(0) as u64,
            covers: intern(rels::FONT_COVERS).unwrap_or(0) as u64,
        }
    }
}

pub struct FontGraph {
    families: BTreeMap<ThingId, FontFamily>,
    faces: BTreeMap<ThingId, FontFace>,
    files: BTreeMap<ThingId, FontFile>,
    superfamilies: BTreeMap<ThingId, FontSuperfamily>,
    family_name_index: BTreeMap<String, ThingId>,
    file_name_index: BTreeMap<String, ThingId>,
    resolve_cache: BTreeMap<ResolveKey, ThingId>,
    font_cache: BTreeMap<ThingId, FontAsset>,
    dirty: bool,
}

impl FontGraph {
    fn new() -> Self {
        Self {
            families: BTreeMap::new(),
            faces: BTreeMap::new(),
            files: BTreeMap::new(),
            superfamilies: BTreeMap::new(),
            family_name_index: BTreeMap::new(),
            file_name_index: BTreeMap::new(),
            resolve_cache: BTreeMap::new(),
            font_cache: BTreeMap::new(),
            dirty: true,
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn refresh_if_needed(&mut self) {
        if !self.dirty {
            return;
        }
        self.refresh();
    }

    fn refresh(&mut self) {
        self.families.clear();
        self.faces.clear();
        self.files.clear();
        self.superfamilies.clear();
        self.family_name_index.clear();
        self.file_name_index.clear();
        self.resolve_cache.clear();
        self.font_cache.clear();
        self.dirty = false;

        let symbols = FontSymbols::intern();

        let file_ids = collect_nodes(kinds::FONT_FILE);
        for file_id in file_ids {
            let bytespace = prop_get(file_id, keys::FONT_BYTESPACE).ok().unwrap_or(0);
            let bytespace_id = ThingId::from_u64(bytespace);
            if bytespace_id.to_u64_lossy() == 0 {
                continue;
            }
            let size = prop_get(file_id, keys::FONT_SIZE_BYTES)
                .ok()
                .map(|v| v as usize)
                .filter(|v| *v > 0)
                .or_else(|| bytespace_info(bytespace_id).ok());
            let name = read_string_prop(file_id, keys::FONT_NAME)
                .unwrap_or_else(|| "font.bin".to_string());
            if let Some(size_bytes) = size {
                self.files.insert(file_id, FontFile {
                    bytespace_id,
                    size_bytes,
                    name: Arc::from(name.as_str()),
                });
            }
        }

        let super_ids = collect_nodes(kinds::FONT_SUPERFAMILY);
        for super_id in super_ids {
            let name = read_string_prop(super_id, keys::FONT_NAME)
                .unwrap_or_else(|| "Superfamily".to_string());
            self.superfamilies.insert(super_id, FontSuperfamily {
                name: Arc::from(name.as_str()),
                families: Vec::new(),
            });
        }

        let family_ids = collect_nodes(kinds::FONT_FAMILY);
        for family_id in family_ids {
            let name = read_string_prop(family_id, keys::FONT_NAME)
                .unwrap_or_else(|| "Family".to_string());
            let lower = name.to_lowercase();
            self.family_name_index.insert(lower, family_id);
            self.families.insert(family_id, FontFamily {
                name: Arc::from(name.as_str()),
                faces: Vec::new(),
                superfamily_id: None,
            });
        }

        for (super_id, superfamily) in self.superfamilies.iter_mut() {
            let mut edges = [abi::types::Edge::default(); 128];
            if let Ok(count) = get_edges(*super_id, &mut edges) {
                for edge in edges.iter().take(count) {
                    if edge.predicate.to_u64_lossy() == symbols.contains {
                        if let Some(family) = self.families.get_mut(&edge.to) {
                            family.superfamily_id = Some(*super_id);
                            superfamily.families.push(edge.to);
                        }
                    }
                }
            }
        }

        let face_ids = collect_nodes(kinds::FONT_FACE);
        for face_id in face_ids {
            let weight = prop_get(face_id, keys::FONT_WEIGHT).ok().unwrap_or(400) as u16;
            let width = prop_get(face_id, keys::FONT_WIDTH).ok().unwrap_or(5) as u16;
            let slope = prop_get(face_id, keys::FONT_SLOPE).ok().unwrap_or(0) as u8;
            let style = read_string_prop(face_id, keys::FONT_STYLE)
                .unwrap_or_else(|| "Regular".to_string());
            self.faces.insert(face_id, FontFace {
                family_id: ThingId::default(),
                file_id: ThingId::default(),
                style: Arc::from(style.as_str()),
                weight,
                width,
                slope,
                coverage: Vec::new(),
            });
        }

        for (family_id, family) in self.families.iter_mut() {
            let mut edges = [abi::types::Edge::default(); 128];
            if let Ok(count) = get_edges(*family_id, &mut edges) {
                for edge in edges.iter().take(count) {
                    if edge.predicate.to_u64_lossy() == symbols.contains {
                        if let Some(face) = self.faces.get_mut(&edge.to) {
                            face.family_id = *family_id;
                            family.faces.push(edge.to);
                        }
                    }
                }
            }
        }

        for (face_id, face) in self.faces.iter_mut() {
            let mut edges = [abi::types::Edge::default(); 128];
            if let Ok(count) = get_edges(*face_id, &mut edges) {
                for edge in edges.iter().take(count) {
                    let pred = edge.predicate.to_u64_lossy();
                    if pred == symbols.contains {
                        if self.files.contains_key(&edge.to) {
                            face.file_id = edge.to;
                        }
                    } else if pred == symbols.covers {
                        if let Some(ranges) = read_coverage(edge.to) {
                            face.coverage = ranges;
                        }
                    }
                }
            }
            if face.file_id.to_u64_lossy() != 0 {
                if let Some(file) = self.files.get(&face.file_id) {
                    let lower = file.name.to_lowercase();
                    self.file_name_index.insert(lower, face.family_id);
                }
            }
        }
    }

    pub fn resolve_stack(&self, stack: Option<&str>) -> Vec<ThingId> {
        let mut resolved = Vec::new();
        let mut tokens = Vec::new();
        if let Some(stack_str) = stack {
            tokens = parse_stack_tokens(stack_str);
        }

        if tokens.is_empty() {
            return self.default_stack();
        }

        for token in tokens {
            if let Some(family_id) = self.find_family_by_name(&token) {
                resolved.push(family_id);
                continue;
            }
            if let Some(family_id) = self.find_family_by_file_name(&token) {
                resolved.push(family_id);
            }
        }

        if resolved.is_empty() {
            self.default_stack()
        } else {
            resolved
        }
    }

    pub fn resolve_face_for_glyph(
        &mut self,
        stack: &[ThingId],
        style: FontStyle,
        codepoint: u32,
    ) -> Option<ResolvedFace> {
        if stack.is_empty() {
            return None;
        }
        let stack_hash = hash_stack(stack);
        let key = ResolveKey::new(stack_hash, style, codepoint);
        if let Some(face_id) = self.resolve_cache.get(&key) {
            return self.resolved_face_by_id(*face_id);
        }

        for family_id in stack {
            if let Some(face_id) = self.resolve_in_family(*family_id, style, codepoint) {
                self.resolve_cache.insert(key, face_id);
                return self.resolved_face_by_id(face_id);
            }
        }

        None
    }

    pub fn select_face_for_family(
        &self,
        family_id: ThingId,
        style: FontStyle,
    ) -> Option<ThingId> {
        let family = self.families.get(&family_id)?;
        best_face_for_style(&family.faces, &self.faces, style)
    }

    pub fn font_for_face(&mut self, face_id: ThingId) -> Option<&FontAsset> {
        let face = self.faces.get(&face_id)?;
        let file = self.files.get(&face.file_id)?;
        if !self.font_cache.contains_key(&face.file_id) {
            if let Some(asset) = AssetBank::load_font_immediate(
                file.bytespace_id,
                file.size_bytes,
                file.name.as_ref(),
            ) {
                self.font_cache.insert(face.file_id, asset);
            }
        }
        self.font_cache.get(&face.file_id)
    }

    pub fn has_fonts(&self) -> bool {
        !self.faces.is_empty()
    }

    fn resolve_in_family(
        &self,
        family_id: ThingId,
        style: FontStyle,
        codepoint: u32,
    ) -> Option<ThingId> {
        if let Some(family) = self.families.get(&family_id) {
            let mut candidates = rank_faces(&family.faces, &self.faces, style);
            if let Some(face_id) = first_face_covering(&candidates, &self.faces, codepoint) {
                return Some(face_id);
            }

            if let Some(super_id) = family.superfamily_id {
                if let Some(superfam) = self.superfamilies.get(&super_id) {
                    for other_family in &superfam.families {
                        if *other_family == family_id {
                            continue;
                        }
                        if let Some(other) = self.families.get(other_family) {
                            candidates = rank_faces(&other.faces, &self.faces, style);
                            if let Some(face_id) = first_face_covering(&candidates, &self.faces, codepoint) {
                                return Some(face_id);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn resolved_face_by_id(&self, face_id: ThingId) -> Option<ResolvedFace> {
        let face = self.faces.get(&face_id)?;
        let family = self.families.get(&face.family_id)?;
        Some(ResolvedFace {
            face_id,
            family_id: face.family_id,
            family_name: family.name.clone(),
            style_name: face.style.clone(),
            file_id: face.file_id,
        })
    }

    fn default_stack(&self) -> Vec<ThingId> {
        let mut stack = Vec::new();
        let preferred = ["Noto Sans", "Noto Sans Symbols", "Noto Sans Symbols 2"];
        for name in preferred {
            if let Some(id) = self.find_family_by_name(name) {
                if !stack.contains(&id) {
                    stack.push(id);
                }
            }
        }
        if stack.is_empty() {
            let mut by_name: Vec<_> = self.families.iter().collect();
            by_name.sort_by_key(|(_, family)| family.name.as_ref().to_lowercase());
            for (id, _) in by_name.into_iter().take(3) {
                stack.push(*id);
            }
        }
        stack
    }

    fn find_family_by_name(&self, name: &str) -> Option<ThingId> {
        self.family_name_index.get(&name.to_lowercase()).copied()
    }

    fn find_family_by_file_name(&self, name: &str) -> Option<ThingId> {
        let needle = name.to_lowercase();
        if let Some(id) = self.file_name_index.get(&needle) {
            return Some(*id);
        }
        for (key, id) in &self.file_name_index {
            if key.ends_with(&needle) {
                return Some(*id);
            }
        }
        None
    }
}

fn collect_nodes(kind: &str) -> Vec<ThingId> {
    let mut out = Vec::new();
    let mut buf = [ThingId::default(); 256];
    if let Ok(count) = find(kind, &mut buf) {
        for id in buf.iter().take(count) {
            out.push(*id);
        }
    }
    out
}

fn read_string_prop(node: ThingId, key: &str) -> Option<String> {
    let val = prop_get(node, key).ok()?;
    if val == 0 {
        return None;
    }
    read_bytespace_string(ThingId::from_u64(val))
}

fn read_bytespace_string(id: ThingId) -> Option<String> {
    let size = bytespace_info(id).ok()?;
    if size == 0 {
        return Some(String::new());
    }
    let mut buf = vec![0u8; size];
    let len = bytespace_read(id, 0, &mut buf).ok()?;
    let text = core::str::from_utf8(&buf[..len]).unwrap_or("");
    Some(text.to_string())
}

fn parse_stack_tokens(stack: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let parts: Vec<&str> = if stack.contains("->") {
        stack.split("->").collect()
    } else if stack.contains(',') {
        stack.split(',').collect()
    } else {
        vec![stack]
    };
    for part in parts {
        let token = part.trim();
        if !token.is_empty() {
            tokens.push(token.to_string());
        }
    }
    tokens
}

fn read_coverage(node: ThingId) -> Option<Vec<(u32, u32)>> {
    let ranges = read_string_prop(node, keys::FONT_COVERAGE_RANGES)?;
    let mut out = Vec::new();
    for part in ranges.split(',') {
        let chunk = part.trim();
        if chunk.is_empty() {
            continue;
        }
        if let Some((a, b)) = chunk.split_once('-') {
            if let (Ok(start), Ok(end)) = (u32::from_str_radix(a, 16), u32::from_str_radix(b, 16)) {
                out.push((start, end));
            }
        } else if let Ok(val) = u32::from_str_radix(chunk, 16) {
            out.push((val, val));
        }
    }
    out.sort_by_key(|(a, _)| *a);
    Some(out)
}

fn first_face_covering(
    faces: &[ThingId],
    face_map: &BTreeMap<ThingId, FontFace>,
    codepoint: u32,
) -> Option<ThingId> {
    for face_id in faces {
        if let Some(face) = face_map.get(face_id) {
            if coverage_contains(&face.coverage, codepoint) {
                return Some(*face_id);
            }
        }
    }
    None
}

fn coverage_contains(ranges: &[(u32, u32)], codepoint: u32) -> bool {
    let mut low = 0usize;
    let mut high = ranges.len();
    while low < high {
        let mid = (low + high) / 2;
        let (start, end) = ranges[mid];
        if codepoint < start {
            high = mid;
        } else if codepoint > end {
            low = mid + 1;
        } else {
            return true;
        }
    }
    false
}

fn rank_faces(
    faces: &[ThingId],
    face_map: &BTreeMap<ThingId, FontFace>,
    style: FontStyle,
) -> Vec<ThingId> {
    let mut ranked = faces.to_vec();
    ranked.sort_by_key(|id| {
        face_map
            .get(id)
            .map(|face| face_style_score(face, style))
            .unwrap_or(i32::MAX)
    });
    ranked
}

fn best_face_for_style(
    faces: &[ThingId],
    face_map: &BTreeMap<ThingId, FontFace>,
    style: FontStyle,
) -> Option<ThingId> {
    faces
        .iter()
        .min_by_key(|id| {
            face_map
                .get(id)
                .map(|face| face_style_score(face, style))
                .unwrap_or(i32::MAX)
        })
        .copied()
}

fn face_style_score(face: &FontFace, style: FontStyle) -> i32 {
    let weight = (face.weight as i32 - style.weight as i32).abs();
    let width = (face.width as i32 - style.width as i32).abs() * 100;
    let slope = if face.slope == style.slope { 0 } else { 1000 };
    weight + width + slope
}

fn hash_stack(stack: &[ThingId]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for id in stack {
        hash = fnv_mix(hash, &id.to_u64_lossy().to_le_bytes());
    }
    hash
}

fn fnv_mix(mut hash: u64, bytes: &[u8]) -> u64 {
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

static FONT_GRAPH: Mutex<Option<FontGraph>> = Mutex::new(None);

pub fn with_graph<F, R>(f: F) -> R
where
    F: FnOnce(&mut FontGraph) -> R,
{
    let mut guard = FONT_GRAPH.lock();
    if guard.is_none() {
        *guard = Some(FontGraph::new());
    }
    let graph = guard.as_mut().expect("font graph");
    graph.refresh_if_needed();
    f(graph)
}

pub fn mark_dirty() {
    let mut guard = FONT_GRAPH.lock();
    if let Some(graph) = guard.as_mut() {
        graph.mark_dirty();
    }
}

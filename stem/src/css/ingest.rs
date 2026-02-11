//! CSS to Graph ingestion.
//!
//! Converts CSS stylesheets into graph nodes using a simple custom tokenizer.
//! This avoids adding external dependencies while handling common CSS patterns.

use crate::css::model::{kinds, props, rels};
use crate::thing::ThingId;
use crate::xml::ingest::GraphApply;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub enum CssIngestError {
    ParseError,
    GraphError,
    CreateNodeFailed,
    LinkFailed,
    SetPropFailed,
    InternFailed,
    LimitExceeded(&'static str),
    Utf8Error,
}

impl From<crate::xml::ingest::XmlIngestError> for CssIngestError {
    fn from(e: crate::xml::ingest::XmlIngestError) -> Self {
        match e {
            crate::xml::ingest::XmlIngestError::CreateNodeFailed => {
                CssIngestError::CreateNodeFailed
            }
            crate::xml::ingest::XmlIngestError::LinkFailed => CssIngestError::LinkFailed,
            crate::xml::ingest::XmlIngestError::SetPropFailed => CssIngestError::SetPropFailed,
            crate::xml::ingest::XmlIngestError::InternFailed => CssIngestError::InternFailed,
            _ => CssIngestError::GraphError,
        }
    }
}

pub struct CssIngestOptions<'a> {
    pub source_name: &'a str,
    pub attach_under: Option<ThingId>,
    pub max_rules: usize,
    pub max_declarations: usize,
}

impl Default for CssIngestOptions<'_> {
    fn default() -> Self {
        Self {
            source_name: "",
            attach_under: None,
            max_rules: 1024,
            max_declarations: 8192,
        }
    }
}

#[derive(Debug, Default)]
pub struct CssIngestResult {
    pub stylesheet: ThingId,
    pub rule_count: u32,
    pub declaration_count: u32,
    pub at_rule_count: u32,
}

/// Ingest a CSS stylesheet into the graph.
pub fn ingest_css_to_graph(
    bytes: &[u8],
    opts: CssIngestOptions<'_>,
    graph: &mut dyn GraphApply,
) -> Result<CssIngestResult, CssIngestError> {
    let css_str = core::str::from_utf8(bytes).map_err(|_| CssIngestError::Utf8Error)?;

    let mut result = CssIngestResult::default();

    // Create Stylesheet Node
    let stylesheet_id = graph.create_node(kinds::STYLESHEET)?;
    let src_sym = graph.intern(opts.source_name)?;
    graph.set_prop(stylesheet_id, props::SOURCE, src_sym)?;

    if let Some(parent) = opts.attach_under {
        graph.link(parent, rels::HAS_CHILD, stylesheet_id)?;
    }

    result.stylesheet = stylesheet_id;

    let mut rule_order: u32 = 0;

    // Simple CSS parser state machine
    let mut chars = css_str.chars().peekable();

    while chars.peek().is_some() {
        skip_whitespace_and_comments(&mut chars);

        if chars.peek().is_none() {
            break;
        }

        // Check for at-rule
        if chars.peek() == Some(&'@') {
            if result.at_rule_count as usize >= opts.max_rules {
                return Err(CssIngestError::LimitExceeded("max_rules"));
            }
            parse_at_rule(&mut chars, stylesheet_id, rule_order, graph, &mut result)?;
            rule_order += 1;
            continue;
        }

        // Parse a regular rule: selector { declarations }
        let selector = parse_until(&mut chars, '{');
        let selector = selector.trim();

        if selector.is_empty() {
            // Skip empty or malformed content
            if chars.peek() == Some(&'{') {
                chars.next();
            }
            skip_block(&mut chars);
            continue;
        }

        if result.rule_count as usize >= opts.max_rules {
            return Err(CssIngestError::LimitExceeded("max_rules"));
        }

        // Skip the '{'
        if chars.next() != Some('{') {
            continue;
        }

        // Create Rule node
        let rule_id = graph.create_node(kinds::RULE)?;
        graph.link(stylesheet_id, rels::HAS_CHILD, rule_id)?;
        graph.set_prop(rule_id, props::ORDER, rule_order as u64)?;
        result.rule_count += 1;
        rule_order += 1;

        // Create Selector node
        let selector_id = graph.create_node(kinds::SELECTOR)?;
        let sel_sym = graph.intern(selector)?;
        graph.set_prop(selector_id, props::SELECTOR_TEXT, sel_sym)?;
        graph.link(rule_id, rels::HAS_CHILD, selector_id)?;

        // Parse declarations
        let declarations_str = parse_until(&mut chars, '}');
        chars.next(); // Skip '}'

        let mut decl_order: u32 = 0;
        for decl in declarations_str.split(';') {
            let decl = decl.trim();
            if decl.is_empty() {
                continue;
            }

            if result.declaration_count as usize >= opts.max_declarations {
                return Err(CssIngestError::LimitExceeded("max_declarations"));
            }

            if let Some((prop, val)) = decl.split_once(':') {
                let prop = prop.trim();
                let val = val.trim();

                let decl_id = graph.create_node(kinds::DECLARATION)?;
                let prop_sym = graph.intern(prop)?;
                let val_sym = graph.intern(val)?;

                graph.set_prop(decl_id, props::PROPERTY, prop_sym)?;
                graph.set_prop(decl_id, props::VALUE, val_sym)?;
                graph.set_prop(decl_id, props::ORDER, decl_order as u64)?;
                graph.link(rule_id, rels::HAS_CHILD, decl_id)?;

                result.declaration_count += 1;
                decl_order += 1;
            }
        }
    }

    Ok(result)
}

fn skip_whitespace_and_comments<I: Iterator<Item = char>>(chars: &mut core::iter::Peekable<I>) {
    loop {
        // Skip whitespace
        while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
            chars.next();
        }

        // Skip /* */ comments - check for start of comment
        if chars.peek() == Some(&'/') {
            // Consume '/' and check if followed by '*'
            chars.next();
            if chars.peek() == Some(&'*') {
                chars.next(); // Skip '*'
                              // Now skip until we see '*/'
                loop {
                    match chars.next() {
                        Some('*') if chars.peek() == Some(&'/') => {
                            chars.next();
                            break;
                        }
                        None => break,
                        _ => {}
                    }
                }
                continue;
            }
            // It was just a '/' not followed by '*', but we consumed it
            // This is a limitation - we can't put it back. Skip for now.
        }

        break;
    }
}

fn parse_until<I: Iterator<Item = char>>(
    chars: &mut core::iter::Peekable<I>,
    delimiter: char,
) -> String {
    let mut result = String::new();
    while let Some(&c) = chars.peek() {
        if c == delimiter {
            break;
        }
        result.push(c);
        chars.next();
    }
    result
}

fn skip_block<I: Iterator<Item = char>>(chars: &mut core::iter::Peekable<I>) {
    let mut depth = 1;
    while let Some(c) = chars.next() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            _ => {}
        }
    }
}

fn parse_at_rule<I: Iterator<Item = char>>(
    chars: &mut core::iter::Peekable<I>,
    stylesheet_id: ThingId,
    order: u32,
    graph: &mut dyn GraphApply,
    result: &mut CssIngestResult,
) -> Result<(), CssIngestError> {
    chars.next(); // Skip '@'

    // Parse at-rule name
    let mut name = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() || c == '{' || c == ';' {
            break;
        }
        name.push(c);
        chars.next();
    }

    skip_whitespace_and_comments(chars);

    // Parse prelude (everything before { or ;)
    let mut prelude = String::new();
    while let Some(&c) = chars.peek() {
        if c == '{' || c == ';' {
            break;
        }
        prelude.push(c);
        chars.next();
    }

    let at_rule_id = graph.create_node(kinds::AT_RULE)?;
    let name_sym = graph.intern(&name)?;
    graph.set_prop(at_rule_id, props::AT_RULE_NAME, name_sym)?;
    graph.set_prop(at_rule_id, props::ORDER, order as u64)?;

    let prelude_trimmed = prelude.trim();
    if !prelude_trimmed.is_empty() {
        let prelude_sym = graph.intern(prelude_trimmed)?;
        graph.set_prop(at_rule_id, props::PRELUDE, prelude_sym)?;
    }

    graph.link(stylesheet_id, rels::HAS_CHILD, at_rule_id)?;
    result.at_rule_count += 1;

    // Handle block or statement
    match chars.peek() {
        Some(&'{') => {
            chars.next();
            skip_block(chars);
        }
        Some(&';') => {
            chars.next();
        }
        _ => {}
    }

    Ok(())
}

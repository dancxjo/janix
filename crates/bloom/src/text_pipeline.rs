//! Text pipeline: try textd result first, fallback to Unifont
//!
//! This module provides a non-blocking text rendering abstraction:
//! - Check if textd service is online (FontFace Things exist)
//! - Create/manage TextRenderRequest Things for text that needs rendering
//! - Blit ready results when available, otherwise use Unifont fallback

use alloc::collections::BTreeMap;
use abi::ids::{SymbolId, ThingId};
use thing_std::graph::{symbol_intern, thing_find, thing_get_body, relationships_from};
use thing_std::SyscallGraphClient;
use models::{FontFace, GlyphCache, TextRenderRequest, TextRenderResult, TextRenderStatus, Thing};

/// Text pipeline state maintained by Bloom
pub struct TextPipeline {
    /// Whether textd is available (any FontFace Things exist)
    pub textd_online: bool,
    /// Cache of request ThingIds by text key hash
    request_cache: BTreeMap<u64, ThingId>,
    /// Default font face (first one found)
    default_font: Option<ThingId>,
    /// Fonts graph ID
    fonts_graph: Option<ThingId>,
    /// Requests graph ID
    requests_graph: Option<ThingId>,
    /// Last check tick for textd presence
    last_check_tick: u64,
}

impl TextPipeline {
    pub fn new() -> Self {
        Self {
            textd_online: false,
            request_cache: BTreeMap::new(),
            default_font: None,
            fonts_graph: None,
            requests_graph: None,
            last_check_tick: 0,
        }
    }

    /// Check if textd is available (called periodically, not every frame)
    pub fn check_textd_online(&mut self, _client: &mut SyscallGraphClient) {
        // Check for fonts graph
        if self.fonts_graph.is_none() {
            self.fonts_graph = thing_find("graph.text.fonts");
        }
        if self.requests_graph.is_none() {
            self.requests_graph = thing_find("graph.text.requests");
        }

        let Some(fonts_graph) = self.fonts_graph else {
            self.textd_online = false;
            return;
        };

        // Look for any FontFace Thing
        let pred_contains = symbol_intern("predicate.contains");
        let mut buf = [abi::types::RelationshipRef {
            id: ThingId(0),
            kind: SymbolId(0),
            target: ThingId(0),
        }; 8];

        if let Ok((n, _)) = relationships_from(fonts_graph, 0, &mut buf) {
            for i in 0..(n as usize) {
                if buf[i].kind == pred_contains {
                    // Check if it's a FontFamily or FontFace
                    if let Some((body, _)) = thing_get_body(buf[i].target) {
                        if FontFace::decode_full(&body).is_ok() {
                            self.textd_online = true;
                            if self.default_font.is_none() {
                                self.default_font = Some(buf[i].target);
                            }
                            return;
                        }
                    }
                }
            }
        }

        self.textd_online = false;
    }

    /// Get the default font face if available
    pub fn default_font(&self) -> Option<ThingId> {
        self.default_font
    }

    /// Check if a result is ready for a text key
    pub fn try_ready_result(&self, _text_key: u64) -> Option<TextRenderResult> {
        // TODO: Implement request lookup and result checking
        // For now, always return None (use fallback)
        None
    }
}

impl Default for TextPipeline {
    fn default() -> Self {
        Self::new()
    }
}

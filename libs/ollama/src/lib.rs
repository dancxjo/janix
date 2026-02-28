#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::task::{Context, Poll};

use llm::{ChatDelta, ChatRequest, ChatStream, FinishReason, LlmError, StreamingLlmClient};

/// Minimal no_std Ollama client shim.
///
/// Thing-OS userspace currently has no direct TCP stack available inside this
/// crate, so streaming requests return a transport error at runtime.
pub struct OllamaClient {
    base_url: String,
    model: String,
}

impl OllamaClient {
    pub fn new(base_url: &str, model: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            model: model.to_string(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn model(&self) -> &str {
        &self.model
    }
}

impl StreamingLlmClient for OllamaClient {
    fn chat_stream(&self, _req: ChatRequest) -> Result<Box<dyn ChatStream + Send>, LlmError> {
        Err(LlmError::Transport(
            "ollama transport unavailable in no_std userspace".to_string(),
        ))
    }
}

/// Helper stream used by tests and future adapters.
pub struct EmptyChatStream {
    done: bool,
}

impl EmptyChatStream {
    pub const fn new() -> Self {
        Self { done: false }
    }
}

impl ChatStream for EmptyChatStream {
    fn poll_next(&mut self, _cx: &mut Context<'_>) -> Poll<Result<Option<ChatDelta>, LlmError>> {
        if self.done {
            return Poll::Ready(Ok(None));
        }
        self.done = true;
        Poll::Ready(Ok(Some(ChatDelta {
            text: String::new(),
            finish: Some(FinishReason::Stop),
        })))
    }
}

#![no_std]
extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use core::task::{Context, Poll};
use serde::{Deserialize, Serialize};

use llm::{ChatDelta, ChatRequest, ChatStream, FinishReason, LlmError, Role, StreamingLlmClient};
use http::{HttpClient, Response};

#[derive(Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    messages: Vec<OllamaMessage<'a>>,
    stream: bool,
}

#[derive(Serialize)]
struct OllamaMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct OllamaResponse {
    message: Option<OllamaResponseMessage>,
    done: bool,
}

#[derive(Deserialize)]
struct OllamaResponseMessage {
    content: String,
}

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
}

impl StreamingLlmClient for OllamaClient {
    fn chat_stream(&self, req: ChatRequest) -> Result<Box<dyn ChatStream + Send>, LlmError> {
        let messages: Vec<OllamaMessage> = req.messages.iter().map(|m| OllamaMessage {
            role: match m.role {
                Role::System => "system",
                Role::User => "user",
                Role::Assistant => "assistant",
            },
            content: &m.content,
        }).collect();

        let ollama_req = OllamaRequest {
            model: &self.model,
            messages,
            stream: true,
        };

        let body = serde_json::to_string(&ollama_req).map_err(|_| LlmError::Other("Serialize error".to_string()))?;
        let url = format!("{}/api/chat", self.base_url);

        let response = HttpClient::post(&url, &body).map_err(|e| LlmError::Transport(e))?;

        Ok(Box::new(OllamaChatStream {
            response,
            buffer: Vec::new(),
            done: false,
        }))
    }
}

struct OllamaChatStream {
    response: Response,
    buffer: Vec<u8>,
    done: bool,
}

impl ChatStream for OllamaChatStream {
    fn poll_next(&mut self, _cx: &mut Context<'_>) -> Poll<Result<Option<ChatDelta>, LlmError>> {
        if self.done && self.buffer.is_empty() {
             return Poll::Ready(Ok(None));
        }

        // Try to read more data
        match self.response.read_chunk() {
            Ok(chunk) => {
                if chunk.is_empty() {
                    // Check if we have anything pending in buffer that couldn't be parsed
                    if self.buffer.is_empty() {
                         return Poll::Ready(Ok(None));
                    }
                    // If we have buffer but stream closed, it might be an error or just incomplete JSON.
                    // We can try to parse one last time or just error/stop.
                }
                self.buffer.extend_from_slice(&chunk);
            }
            Err(e) => return Poll::Ready(Err(LlmError::Transport(e))),
        }

        let mut consumed = 0;
        let mut result = None;

        {
            let mut stream = serde_json::Deserializer::from_slice(&self.buffer).into_iter::<OllamaResponse>();

            while let Some(Ok(resp)) = stream.next() {
                consumed = stream.byte_offset();

                if resp.done {
                    self.done = true;
                    // FinishReason::Stop
                    // We might have content too? Usually done message has empty content or stats.
                    // If we have message content, yield it first?
                    // But we can only return one delta.
                    // Let's assume we return empty text with Stop.
                    result = Some(ChatDelta {
                         text: String::new(),
                         finish: Some(FinishReason::Stop),
                    });
                    break;
                }

                if let Some(msg) = resp.message {
                    result = Some(ChatDelta {
                        text: msg.content,
                        finish: None,
                    });
                    break;
                }
            }
        }

        if consumed > 0 {
            self.buffer.drain(..consumed);
        }

        if let Some(delta) = result {
            return Poll::Ready(Ok(Some(delta)));
        }

        if self.done {
             return Poll::Ready(Ok(None));
        }

        // If we still have data in buffer but couldn't parse, we need more data.
        // Or if buffer is empty, we need more data.
        Poll::Pending
    }
}

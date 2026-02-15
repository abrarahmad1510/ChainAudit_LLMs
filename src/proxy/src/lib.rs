use proxy_wasm as wasm;
use wasm::traits::*;
use wasm::types::*;
use serde_json::{json, Value};
use std::collections::VecDeque;

mod canonical;
use canonical::canonicalise;

// Root context – created once per filter instance
struct ProxyRoot;

impl Context for ProxyRoot {}

impl RootContext for ProxyRoot {
    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(ProxyHttp::default()))
    }
}

// Per‑stream HTTP context
#[derive(Default)]
struct ProxyHttp {
    request_headers: Vec<(String, String)>,
    response_body: Option<Vec<u8>>,
    hash_queue: VecDeque<Vec<u8>>,
}

impl Context for ProxyHttp {}

impl HttpContext for ProxyHttp {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        let headers = self.get_http_request_headers();
        self.request_headers = headers
            .into_iter()
            .filter(|(k, _)| {
                k == "x-model-id"
                    || k == "x-adapter-ids"
                    || k == "x-prompt-template-id"
                    || k == "x-approval-status"
            })
            .collect();
        Action::Continue
    }

    fn on_http_response_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        if end_of_stream {
            if let Some(body) = self.get_http_response_body(0, body_size) {
                self.response_body = Some(body.to_vec());
                let context = self.build_context();
                let canonical = canonicalise(&context);
                let hash = blake3::hash(&canonical).as_bytes().to_vec();
                self.hash_queue.push_back(hash.clone());

                wasm::hostcalls::log(
                    wasm::types::LogLevel::Info,
                    &format!("Computed hash: {}", hex::encode(&hash)),
                )
                .unwrap();
            }
        }
        Action::Continue
    }
}

impl ProxyHttp {
    fn build_context(&self) -> Value {
        let request_headers_obj: Value = self
            .request_headers
            .iter()
            .map(|(k, v)| (k.clone(), Value::String(v.clone())))
            .collect();

        let response_body_str = self
            .response_body
            .as_deref()
            .map(|b| String::from_utf8_lossy(b).to_string())
            .unwrap_or_default();

        json!({
            "request_headers": request_headers_obj,
            "response_text": response_body_str,
            "timestamp_ns": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        })
    }
}

#[no_mangle]
pub fn _start() {
    wasm::set_root_context(|_| Box::new(ProxyRoot));
}

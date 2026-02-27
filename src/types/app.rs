use std::{
    borrow::Cow,
    sync::mpsc::{Receiver, Sender},
    time::Instant,
};

use curl_rest::{Client, Header, Method, QueryParam, Response};
use ratatui::crossterm::event::KeyEvent;
use tui_input::{Input, InputRequest};

use crate::types::input_handler::{InputHandler, InputState};
use crate::ui::sections::response_body::{ResponseBody, format_for_display};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Normal,
    Edit,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum BodyMode {
    #[default]
    Json,
    Form,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ActivePanel {
    #[default]
    Url,
    ReqQuery,
    ReqHeaders,
    ReqBody,
    ResHeaders,
    ResBody,
}

impl ActivePanel {
    pub fn next(self) -> Self {
        match self {
            Self::Url => Self::ReqQuery,
            Self::ReqQuery => Self::ReqHeaders,
            Self::ReqHeaders => Self::ReqBody,
            Self::ReqBody => Self::ResHeaders,
            Self::ResHeaders => Self::ResBody,
            Self::ResBody => Self::Url,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Url => Self::ResBody,
            Self::ReqQuery => Self::Url,
            Self::ReqHeaders => Self::ReqQuery,
            Self::ReqBody => Self::ReqHeaders,
            Self::ResHeaders => Self::ReqBody,
            Self::ResBody => Self::ResHeaders,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub should_exit: bool,
    pub mode: Mode,
    pub active_panel: ActivePanel,
    pub body_mode: BodyMode,
    pub(crate) is_loading: bool,
    pub error: Option<String>,
    pub response_body: Option<String>,
    pub response_status: Option<String>,
    pub response_headers: Option<String>,
    pub response_content_type: Option<String>,
    pub response_formatted_body: Option<String>,
    pub response_scroll: u16,
    pub response_scroll_x: u16,
    pub response_viewport_height: u16,
    pub response_viewport_width: u16,
    pub response_line_count: usize,
    pub response_max_line_width: usize,
    pub response_max_line_width_cache: Option<usize>,
    pub(crate) response_time: u128,
}

#[derive(Debug, Default, Clone)]
struct Request {
    method: Method,
    body: String,
}

enum BodyPayload {
    Json(String),
    Form(String),
}

pub struct App<'a> {
    pub app_state: AppState,
    pub url_input: Input,
    pub req_query_input: Input,
    pub req_headers_input: Input,
    pub req_body_input: Input,
    pub network: Client<'a>,
    request: Request,
    request_tx: Sender<Response>,
    request_rx: Receiver<Response>,
    his_tx: Sender<Request>,
    err_tx: Sender<String>,
    err_rx: Receiver<String>,
    elapsed_tx: Sender<u128>,
    elapsed_rx: Receiver<u128>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        let (request_tx, request_rx) = std::sync::mpsc::channel::<Response>();
        let (his_tx, _his_rx) = std::sync::mpsc::channel::<Request>();
        let (err_tx, err_rx) = std::sync::mpsc::channel::<String>();
        let (elapsed_tx, elapsed_rx) = std::sync::mpsc::channel::<u128>();

        Self {
            app_state: Default::default(),
            url_input: Default::default(),
            req_query_input: Default::default(),
            req_headers_input: Default::default(),
            req_body_input: Default::default(),
            network: Default::default(),
            request: Default::default(),
            request_tx,
            request_rx,
            his_tx,
            err_tx,
            err_rx,
            elapsed_tx,
            elapsed_rx,
        }
    }
}

impl<'a> App<'a> {
    pub(crate) fn with_default_url(mut self, url: &str) -> Self {
        url.chars().for_each(|char| {
            self.url_input.handle(InputRequest::InsertChar(char));
        });
        self
    }

    pub(crate) fn handle_key_events(&mut self, key_event: KeyEvent) {
        let mut input_handler = InputHandler::new(self, InputState::default());
        input_handler.handle(key_event);
    }

    pub(crate) fn method_label(&self) -> &'static str {
        match self.request.method {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Patch => "PATCH",
            Method::Head => "HEAD",
            Method::Options => "OPTIONS",
            Method::Connect => "CONNECT",
            Method::Trace => "TRACE",
        }
    }

    pub(crate) fn next_method(&mut self) {
        self.request.method = match self.request.method {
            Method::Get => Method::Post,
            Method::Post => Method::Put,
            Method::Put => Method::Delete,
            Method::Delete => Method::Patch,
            Method::Patch => Method::Head,
            Method::Head => Method::Options,
            Method::Options => Method::Connect,
            Method::Connect => Method::Trace,
            Method::Trace => Method::Get,
        };
    }

    pub(crate) fn prev_method(&mut self) {
        self.request.method = match self.request.method {
            Method::Get => Method::Trace,
            Method::Post => Method::Get,
            Method::Put => Method::Post,
            Method::Delete => Method::Put,
            Method::Patch => Method::Delete,
            Method::Head => Method::Patch,
            Method::Options => Method::Head,
            Method::Connect => Method::Options,
            Method::Trace => Method::Connect,
        };
    }

    pub(crate) fn body_content_type(&self) -> &'static str {
        match self.app_state.body_mode {
            BodyMode::Json => "application/json",
            BodyMode::Form => "application/x-www-form-urlencoded",
        }
    }

    pub(crate) fn next_body_mode(&mut self) {
        self.app_state.body_mode = match self.app_state.body_mode {
            BodyMode::Json => BodyMode::Form,
            BodyMode::Form => BodyMode::Json,
        };
    }

    pub(crate) fn prev_body_mode(&mut self) {
        self.app_state.body_mode = match self.app_state.body_mode {
            BodyMode::Json => BodyMode::Form,
            BodyMode::Form => BodyMode::Json,
        };
    }

    pub(crate) fn send_request(&mut self) {
        self.app_state.is_loading = true;
        self.app_state.error = None;
        self.app_state.response_body = None;
        self.app_state.response_status = None;
        self.app_state.response_headers = None;
        self.app_state.response_content_type = None;
        self.app_state.response_formatted_body = None;
        self.app_state.response_max_line_width_cache = None;
        self.app_state.response_scroll = 0;
        self.app_state.response_scroll_x = 0;
        self.refresh_response_body_cache();

        let request = self.request.clone();
        let method = self.request.method.clone();
        let url = self.url_input.value().to_string();
        let _body = self.request.body.to_string();
        let request_tx = self.request_tx.clone();
        let his_tx = self.his_tx.clone();
        let error_tx = self.err_tx.clone();
        let elapsed_tx = self.elapsed_tx.clone();

        let query_params = parse_query_params(self.req_query_input.value());
        let mut headers = parse_headers(self.req_headers_input.value());
        let body_mode = self.app_state.body_mode;
        let body_raw = self.req_body_input.value().to_string();
        let body_payload = match body_mode {
            BodyMode::Json => {
                let pairs = parse_key_value_lines(&body_raw);
                let json = json_from_pairs(pairs);
                json.map(BodyPayload::Json)
            }
            BodyMode::Form => {
                let pairs = parse_key_value_lines(&body_raw);
                let encoded = form_encode(pairs);
                if encoded.is_empty() {
                    None
                } else {
                    Some(BodyPayload::Form(encoded))
                }
            }
        };

        if matches!(body_payload, Some(BodyPayload::Form(_))) && !has_content_type(&headers) {
            headers.push(Header::ContentType(Cow::Borrowed(
                "application/x-www-form-urlencoded",
            )));
        }

        let needs_brotli = headers.iter().any(|header| match header {
            Header::Custom(key, val) => {
                key.eq_ignore_ascii_case("Accept-Encoding") && val.eq_ignore_ascii_case("br")
            }
            _ => false,
        });

        std::thread::spawn(move || {
            let start_time = Instant::now();
            let mut http_client: Client<'static> = Client::default()
                .query_params(query_params)
                .brotli(needs_brotli)
                .headers(headers);
            if let Some(body_payload) = body_payload {
                http_client = match body_payload {
                    BodyPayload::Json(body) => http_client.body_json(body),
                    BodyPayload::Form(body) => http_client.body_text(body),
                };
            }
            let res = match method {
                Method::Get => http_client.get().send(url.as_str()),
                Method::Post => http_client.post().send(url.as_str()),
                Method::Put => http_client.put().send(url.as_str()),
                Method::Delete => http_client.delete().send(url.as_str()),
                Method::Patch => http_client.patch().send(url.as_str()),
                Method::Head => http_client.head().send(url.as_str()),
                Method::Options => http_client.options().send(url.as_str()),
                Method::Connect => http_client.connect().send(url.as_str()),
                Method::Trace => http_client.trace().send(url.as_str()),
            };

            let elapsed = start_time.elapsed().as_millis();

            match res {
                Ok(res) => {
                    let resp = Response {
                        body: res.body,
                        status: res.status,
                        headers: res.headers,
                    };
                    let _ = request_tx.send(resp);
                    let _ = his_tx.send(request);
                    let _ = elapsed_tx.send(elapsed);
                }
                Err(err) => {
                    let message = match err {
                        curl_rest::Error::Client(error) => {
                            format!("Curl error {}", error)
                        }
                        curl_rest::Error::InvalidUrl(url) => {
                            format!("The provided url: {url} is invalid.")
                        }
                        curl_rest::Error::InvalidHeaderValue(v) => {
                            format!("Invalid header value: {v} supplied.")
                        }
                        curl_rest::Error::InvalidHeaderName(n) => {
                            format!("Invalid header name: {n}.")
                        }
                        curl_rest::Error::InvalidStatusCode(c) => {
                            format!("Invalid status code: {c}, supplied.")
                        }
                        curl_rest::Error::BrotliDecompression(e) => {
                            format!("Failed to deceompress brotli: {e}")
                        }
                    };
                    let _ = error_tx.send(message.to_string());
                }
            }
        });
    }

    pub(crate) fn poll_network(&mut self) {
        if let Ok(response) = self.request_rx.try_recv() {
            self.app_state.is_loading = false;
            self.app_state.response_status = Some(response.status.to_string());
            self.app_state.response_body =
                Some(String::from_utf8_lossy(&response.body).to_string());
            self.app_state.response_content_type = response
                .headers
                .iter()
                .find(|h| h.name.eq_ignore_ascii_case("content-type"))
                .map(|h| h.value.to_string());

            if response.headers.is_empty() {
                self.app_state.response_headers = None;
            } else {
                let headers = response
                    .headers
                    .iter()
                    .map(|h| format!("{}: {}", h.name, h.value))
                    .collect::<Vec<_>>()
                    .join("\n");
                self.app_state.response_headers = Some(headers);
            }
            self.app_state.response_scroll = 0;
            self.app_state.response_scroll_x = 0;
            self.refresh_response_body_cache();
            return;
        }

        if let Ok(message) = self.err_rx.try_recv() {
            self.app_state.is_loading = false;
            self.app_state.error = Some(message);
        }

        if let Ok(request_time) = self.elapsed_rx.try_recv() {
            self.app_state.response_time = request_time
        }
    }

    fn refresh_response_body_cache(&mut self) {
        let body = self.app_state.response_body.as_deref();
        let content_type = self.app_state.response_content_type.as_deref();
        self.app_state.response_formatted_body = format_for_display(body, content_type);
        let formatted = self.app_state.response_formatted_body.as_deref();
        self.app_state.response_line_count = ResponseBody.line_count(body, formatted, content_type);
        let max_line_width = ResponseBody.max_line_width(body, formatted, content_type, None);
        self.app_state.response_max_line_width = max_line_width;
        self.app_state.response_max_line_width_cache = Some(max_line_width);
    }
}

fn parse_key_value_lines(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (key, value) = line.split_once(':')?;
            let key = key.trim();
            if key.is_empty() {
                return None;
            }
            let value = value.trim();
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}

fn form_encode(pairs: Vec<(String, String)>) -> String {
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    for (key, value) in pairs {
        serializer.append_pair(&key, &value);
    }
    serializer.finish()
}

fn json_from_pairs(pairs: Vec<(String, String)>) -> Option<String> {
    if pairs.is_empty() {
        return None;
    }
    let mut map = serde_json::Map::new();
    for (key, value) in pairs {
        map.insert(key, serde_json::Value::String(value));
    }
    serde_json::to_string(&serde_json::Value::Object(map)).ok()
}

fn parse_query_params(input: &str) -> Vec<QueryParam<'static>> {
    parse_key_value_lines(input)
        .into_iter()
        .map(|(key, value)| QueryParam::new(key, value))
        .collect()
}

fn parse_headers(input: &str) -> Vec<Header<'static>> {
    parse_key_value_lines(input)
        .into_iter()
        .map(|(key, value)| Header::Custom(Cow::Owned(key), Cow::Owned(value)))
        .collect()
}

fn has_content_type(headers: &[Header<'static>]) -> bool {
    headers.iter().any(|header| match header {
        Header::ContentType(_) => true,
        Header::Custom(name, _) => name.eq_ignore_ascii_case("content-type"),
        _ => false,
    })
}

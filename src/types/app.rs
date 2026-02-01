use std::{
    borrow::Cow,
    sync::mpsc::{Receiver, Sender},
};

use curl_rest::{Client, Header, Method, QueryParam, Response};
use ratatui::crossterm::event::KeyEvent;
use tui_input::Input;

use crate::types::input_handler::{InputHandler, InputState};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Normal,
    Edit,
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
            Self::Url => Self::ReqQuery,
            Self::ReqQuery => Self::Url,
            Self::ReqHeaders => Self::ReqQuery,
            Self::ReqBody => Self::ReqHeaders,
            Self::ResHeaders => Self::ReqBody,
            Self::ResBody => Self::Url,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub should_exit: bool,
    pub mode: Mode,
    pub active_panel: ActivePanel,
    pub(crate) is_loading: bool,
    pub error: Option<String>,
    pub response_body: Option<String>,
    pub response_status: Option<String>,
    pub response_headers: Option<String>,
    pub response_content_type: Option<String>,
    pub response_scroll: u16,
    pub response_viewport_height: u16,
    pub response_line_count: usize,
}

#[derive(Debug, Default, Clone)]
struct Request {
    method: Method,
    body: String,
}

pub struct App<'a> {
    pub app_state: AppState,
    pub url_input: Input,
    pub req_query_input: Input,
    pub req_headers_input: Input,
    pub network: Client<'a>,
    request: Request,
    request_tx: Sender<Response>,
    request_rx: Receiver<Response>,
    his_tx: Sender<Request>,
    err_tx: Sender<String>,
    err_rx: Receiver<String>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        let (request_tx, request_rx) = std::sync::mpsc::channel::<Response>();
        let (his_tx, _his_rx) = std::sync::mpsc::channel::<Request>();
        let (err_tx, err_rx) = std::sync::mpsc::channel::<String>();
        Self {
            app_state: Default::default(),
            url_input: Default::default(),
            req_query_input: Default::default(),
            req_headers_input: Default::default(),
            network: Default::default(),
            request: Default::default(),
            request_tx,
            request_rx,
            his_tx,
            err_tx,
            err_rx,
        }
    }
}

impl<'a> App<'a> {
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

    pub(crate) fn send_request(&mut self) {
        self.app_state.is_loading = true;
        self.app_state.error = None;
        self.app_state.response_body = None;
        self.app_state.response_status = None;
        self.app_state.response_headers = None;
        self.app_state.response_content_type = None;
        self.app_state.response_scroll = 0;

        let request = self.request.clone();
        let method = self.request.method.clone();
        let url = self.url_input.value().to_string();
        let _body = self.request.body.to_string();
        let request_tx = self.request_tx.clone();
        let his_tx = self.his_tx.clone();
        let error_tx = self.err_tx.clone();

        let query_params = parse_query_params(self.req_query_input.value());
        let headers = parse_headers(self.req_headers_input.value());

        std::thread::spawn(move || {
            let http_client: Client<'static> = Client::default()
                .query_params(query_params)
                .headers(headers);
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

            match res {
                Ok(res) => {
                    let resp = Response {
                        body: res.body,
                        status: res.status,
                        headers: res.headers,
                    };
                    let _ = request_tx.send(resp);
                    let _ = his_tx.send(request);
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
                            format!("Invalid header value: {v} supplied supplied.")
                        }
                        curl_rest::Error::InvalidHeaderName(n) => {
                            format!("Invalid header name: {n}.")
                        }
                        curl_rest::Error::InvalidStatusCode(c) => {
                            format!("Invalid status code: {c}, supplied.")
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
            return;
        }

        if let Ok(message) = self.err_rx.try_recv() {
            self.app_state.is_loading = false;
            self.app_state.error = Some(message);
        }
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

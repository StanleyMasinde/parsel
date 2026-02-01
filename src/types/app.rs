use std::sync::mpsc::{Receiver, Sender};

use curl_rest::{Client, Method, Response};
use ratatui::{
    crossterm::event::KeyEvent,
    widgets::{Paragraph, Widget},
};
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
            Self::ResBody => Self::ResHeaders,
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
}

#[derive(Debug, Default, Clone)]
struct Request {
    method: Method,
    body: String,
}

pub struct App<'a> {
    pub app_state: AppState,
    pub url_input: Input,
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

    pub(crate) fn send_request(&mut self) {
        self.app_state.is_loading = true;
        self.app_state.error = None;
        self.app_state.response_body = None;
        self.app_state.response_status = None;
        self.app_state.response_headers = None;

        let request = self.request.clone();
        let method = self.request.method.clone();
        let url = self.url_input.value().to_string();
        let _body = self.request.body.to_string();
        let request_tx = self.request_tx.clone();
        let his_tx = self.his_tx.clone();
        let error_tx = self.err_tx.clone();

        let _query_params = "foo?bar";
        let _headers = "foo";

        std::thread::spawn(move || {
            let http_client: Client<'static> = Client::default();
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
            return;
        }

        if let Ok(message) = self.err_rx.try_recv() {
            self.app_state.is_loading = false;
            self.app_state.error = Some(message);
        }
    }
}

impl<'a> Widget for &App<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new("Parsel").centered().render(area, buf);
    }
}

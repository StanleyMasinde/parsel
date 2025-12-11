mod keyboard;
mod renderer;
mod types;

use std::{sync::Arc, time::Duration};

use ratatui::crossterm::{
    self,
    event::{self, Event, KeyEventKind},
};
use tui_textarea::TextArea;

use crate::{
    http::{self, RestClient},
    ui::{
        keyboard::handle_key,
        renderer::render,
        types::{HttpMethod, Mode, Panel, Request, Response},
    },
};

#[derive(Debug)]
struct App<'a> {
    request: Request,
    response: Option<Response>,
    history: Vec<Request>,
    active_panel: Panel,
    should_quit: bool,
    is_loading: bool,
    error: Option<String>,
    mode: Mode,
    tx: std::sync::mpsc::Sender<Response>,
    rx: std::sync::mpsc::Receiver<Response>,
    err_tx: std::sync::mpsc::Sender<String>,
    err_rx: std::sync::mpsc::Receiver<String>,
    his_tx: std::sync::mpsc::Sender<Request>,
    his_rx: std::sync::mpsc::Receiver<Request>,
    http_client: http::HttpClient,
    url_input: TextArea<'a>,
    response_scroll: u16,
    query_params_input: TextArea<'a>,
    edit_modal: bool,
    headers_input: TextArea<'a>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            method: HttpMethod::GET,
            body: "".into(),
        }
    }
}

impl<'a> App<'a> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let (err_tx, err_rx) = std::sync::mpsc::channel::<String>();
        let (his_tx, his_rx) = std::sync::mpsc::channel::<Request>();
        let http_client = http::HttpClient::default();
        let url_input = TextArea::from("https://httpbin.io/anything".lines());
        let query_params_input = TextArea::default();
        let headers_input = TextArea::default();

        Self {
            request: Request::default(),
            response: None,
            history: vec![],
            active_panel: Panel::Url,
            should_quit: false,
            is_loading: false,
            error: None,
            mode: Mode::Normal,
            tx,
            rx,
            err_rx,
            err_tx,
            his_tx,
            his_rx,
            http_client,
            url_input,
            query_params_input,
            response_scroll: 0,
            edit_modal: false,
            headers_input,
        }
    }

    fn send_request(&mut self) {
        self.is_loading = true;

        let request = self.request.clone();
        let method = self.request.method.clone();
        let url = self.url_input.lines().join("\n");
        let body = self.request.body.to_string();
        let tx = self.tx.clone();
        let his_tx = self.his_tx.clone();
        let error_tx = self.err_tx.clone();

        let query_params = Arc::new(self.query_params_input.lines());
        let headers = self.headers_input.lines();

        let http_client = Arc::new(
            self.http_client
                .clone()
                .with_query_params(query_params.to_vec())
                .with_request_headers(headers.to_vec()),
        );

        std::thread::spawn(move || {
            let json_body = serde_json::from_str(&body.replace("\n", "")).unwrap_or(None);
            let res = match method {
                HttpMethod::GET => http_client.get(&url),
                HttpMethod::POST => http_client.post(&url, json_body),
                HttpMethod::PUT => http_client.put(&url, json_body),
                HttpMethod::DELETE => http_client.delete(&url),
                HttpMethod::PATCH => http_client.patch(&url, json_body),
                HttpMethod::HEAD => http_client.get(&url),
                HttpMethod::OPTIONS => http_client.get(&url),
            };

            match res {
                Ok(res) => {
                    let resp = Response {
                        status_code: res.status.into(),
                        status_text: res.status_text,
                        headers: res.headers,
                        body: res.body,
                        duration_ms: res.elapsed,
                    };
                    tx.send(resp).unwrap();
                    his_tx.send(request).unwrap();
                }
                Err(err) => {
                    let mut message = "Could not make the request due to an unknown reason";
                    if err.is_builder() {
                        message = "Failed to make the request please check the URL";
                    } else if err.is_connect() {
                        message = "Failed to connect to the internet, please check your connection";
                    }
                    let _ = error_tx.send(message.to_string());
                }
            }
        });
    }
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    loop {
        terminal.draw(|frame| render(&mut app, frame))?;

        if crossterm::event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            handle_key(&mut app, key);
        }

        if let Ok(resp) = app.rx.try_recv() {
            app.response = Some(resp);
            app.is_loading = false;
        }

        if let Ok(err) = app.err_rx.try_recv() {
            app.error = Some(err);
            app.response = None;
            app.is_loading = false;
        }

        if let Ok(req) = app.his_rx.try_recv() {
            app.history.push(req);
        }

        if app.should_quit {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}

use std::{collections::HashMap, time::Duration};

use ratatui::{
    Frame,
    crossterm::{
        self,
        event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    },
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
};
use tui_input::{Input, InputRequest};

use crate::http::{self, RestClient};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl HttpMethod {
    fn next(&self) -> Self {
        match self {
            HttpMethod::GET => HttpMethod::POST,
            HttpMethod::POST => HttpMethod::PUT,
            HttpMethod::PUT => HttpMethod::DELETE,
            HttpMethod::DELETE => HttpMethod::PATCH,
            HttpMethod::PATCH => HttpMethod::HEAD,
            HttpMethod::HEAD => HttpMethod::OPTIONS,
            HttpMethod::OPTIONS => HttpMethod::GET,
        }
    }

    fn prev(&self) -> Self {
        match self {
            HttpMethod::GET => HttpMethod::OPTIONS,
            HttpMethod::POST => HttpMethod::GET,
            HttpMethod::PUT => HttpMethod::POST,
            HttpMethod::DELETE => HttpMethod::PUT,
            HttpMethod::PATCH => HttpMethod::DELETE,
            HttpMethod::HEAD => HttpMethod::PATCH,
            HttpMethod::OPTIONS => HttpMethod::HEAD,
        }
    }
}

#[derive(Debug, Clone)]
struct Request {
    method: HttpMethod,
    url: Input,
    headers: Vec<(String, String)>,
    query_params: Vec<(String, String)>,
    body: Input,
}

#[derive(Debug, Clone)]
struct Response {
    status_code: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: String,
    duration_ms: u128,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status_code: 200,
            status_text: "Ok".to_string(),
            headers: HashMap::new(),
            body: Default::default(),
            duration_ms: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Panel {
    Url,
    QueryParams,
    Headers,
    Body,
    Response,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    Normal,
    Edit,
    HeaderEdit,
    QueryParamEdit,
}

#[derive(Debug)]
struct App {
    request: Request,
    response: Option<Response>,
    history: Vec<Request>,
    active_panel: Panel,
    should_quit: bool,
    is_loading: bool,
    error: Option<String>,
    mode: Mode,
    selected_header: usize,
    editing_header_key: bool,
    new_header_key: String,
    new_header_value: String,
    tx: std::sync::mpsc::Sender<Response>,
    rx: std::sync::mpsc::Receiver<Response>,
    err_tx: std::sync::mpsc::Sender<String>,
    err_rx: std::sync::mpsc::Receiver<String>,
    his_tx: std::sync::mpsc::Sender<Request>,
    his_rx: std::sync::mpsc::Receiver<Request>,
    http_client: http::HttpClient,
    new_query_param_value: String,
    editing_query_param_key: bool,
    new_query_param_key: String,
    selected_query_param: usize,
}

impl Default for Request {
    fn default() -> Self {
        let app_name = env!("CARGO_PKG_NAME");
        let app_version = env!("CARGO_PKG_VERSION");
        Self {
            method: HttpMethod::GET,
            url: "https://api.restful-api.dev/objects".into(),
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                (
                    "User-Agent".to_string(),
                    format!("{}/{}", app_name, app_version),
                ),
            ],
            body: "".into(),
            query_params: vec![],
        }
    }
}

impl App {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let (err_tx, err_rx) = std::sync::mpsc::channel::<String>();
        let (his_tx, his_rx) = std::sync::mpsc::channel::<Request>();
        let http_client = http::HttpClient::default();

        Self {
            request: Request::default(),
            response: None,
            history: vec![],
            active_panel: Panel::Url,
            should_quit: false,
            is_loading: false,
            error: None,
            mode: Mode::Normal,
            selected_header: 0,
            editing_header_key: true,
            new_header_key: String::new(),
            new_header_value: String::new(),
            tx,
            rx,
            err_rx,
            err_tx,
            his_tx,
            his_rx,
            http_client,
            new_query_param_value: String::new(),
            editing_query_param_key: false,
            new_query_param_key: String::new(),
            selected_query_param: 0,
        }
    }

    fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        self.error = None;
        match (self.mode, key, modifiers) {
            // Global quit
            (_, KeyCode::Char('c'), KeyModifiers::CONTROL) => self.should_quit = true,
            (Mode::Normal, KeyCode::Char('q'), _) => self.should_quit = true,

            // Mode transitions
            (Mode::Normal, KeyCode::Char('i'), _) => {
                self.mode = Mode::Edit;
            }
            (Mode::Edit, KeyCode::Esc, _) | (Mode::HeaderEdit, KeyCode::Esc, _) => {
                self.mode = Mode::Normal;
                self.new_header_key.clear();
                self.new_header_value.clear();
            }

            // Normal mode navigation
            (Mode::Normal, KeyCode::Char('h'), _) => self.move_left(),
            (Mode::Normal, KeyCode::Char('j'), _) => self.move_down(),
            (Mode::Normal, KeyCode::Char('k'), _) => self.move_up(),
            (Mode::Normal, KeyCode::Char('l'), _) => self.move_right(),
            (Mode::Normal, KeyCode::Tab, _) => self.next_panel(),
            (Mode::Normal, KeyCode::BackTab, _) => self.prev_panel(),

            // Method cycling
            (Mode::Normal, KeyCode::Char('m'), _) if self.active_panel == Panel::Url => {
                self.request.method = self.request.method.next();
            }
            (Mode::Normal, KeyCode::Char('M'), _) if self.active_panel == Panel::Url => {
                self.request.method = self.request.method.prev();
            }

            // Query Param management
            (Mode::Normal, KeyCode::Char('a'), _) if self.active_panel == Panel::QueryParams => {
                self.mode = Mode::QueryParamEdit;
                self.editing_query_param_key = true;
            }
            (Mode::Normal, KeyCode::Char('d'), _) if self.active_panel == Panel::QueryParams => {
                self.delete_query_param();
            }

            // Header management
            (Mode::Normal, KeyCode::Char('a'), _) if self.active_panel == Panel::Headers => {
                self.mode = Mode::HeaderEdit;
                self.editing_header_key = true;
            }
            (Mode::Normal, KeyCode::Char('d'), _) if self.active_panel == Panel::Headers => {
                self.delete_header();
            }

            // Send request
            (Mode::Normal, KeyCode::Enter, _) => self.send_request(),

            // Edit mode - text editing
            (Mode::Edit, KeyCode::Char(c), _) => self.handle_char_input(c),
            (Mode::Edit, KeyCode::Backspace, _) => self.handle_backspace(),
            (Mode::Edit, KeyCode::Delete, _) => self.handle_delete(),
            (Mode::Edit, KeyCode::Left, _) => self.handle_cursor_left(),
            (Mode::Edit, KeyCode::Right, _) => self.handle_cursor_right(),
            (Mode::Edit, KeyCode::Home, _) => self.handle_cursor_home(),
            (Mode::Edit, KeyCode::End, _) => self.handle_cursor_end(),
            (Mode::Edit, KeyCode::Enter, _) => self.handle_enter_input(),

            // Query Param edit mode
            (Mode::QueryParamEdit, KeyCode::Char(c), _) => self.handle_query_param_char_input(c),
            (Mode::QueryParamEdit, KeyCode::Backspace, _) => self.handle_query_param_backspace(),
            (Mode::QueryParamEdit, KeyCode::Tab, _) => {
                if self.editing_query_param_key {
                    self.editing_header_key = false;
                } else {
                    self.add_query_param();
                    self.mode = Mode::Normal;
                }
            }

            (Mode::QueryParamEdit, KeyCode::Enter, _) => {
                self.add_query_param();
                self.mode = Mode::Normal;
            }

            // Header edit mode
            (Mode::HeaderEdit, KeyCode::Char(c), _) => self.handle_header_char_input(c),
            (Mode::HeaderEdit, KeyCode::Backspace, _) => self.handle_header_backspace(),
            (Mode::HeaderEdit, KeyCode::Tab, _) => {
                if self.editing_header_key {
                    self.editing_header_key = false;
                } else {
                    self.add_header();
                    self.mode = Mode::Normal;
                }
            }
            (Mode::HeaderEdit, KeyCode::Enter, _) => {
                self.add_header();
                self.mode = Mode::Normal;
            }

            _ => {}
        }
    }

    fn move_left(&mut self) {
        match self.active_panel {
            Panel::QueryParams if !self.request.query_params.is_empty() => {
                if self.selected_query_param > 0 {
                    self.selected_query_param -= 1;
                }
            }
            Panel::Headers if !self.request.headers.is_empty() => {
                if self.selected_header > 0 {
                    self.selected_header -= 1;
                }
            }
            _ => {}
        }
    }

    fn move_right(&mut self) {
        match self.active_panel {
            Panel::QueryParams if !self.request.query_params.is_empty() => {
                if self.selected_query_param < self.request.query_params.len() - 1 {
                    self.selected_query_param += 1;
                }
            }
            Panel::Headers if !self.request.headers.is_empty() => {
                if self.selected_header < self.request.headers.len() - 1 {
                    self.selected_header += 1;
                }
            }
            _ => {}
        }
    }

    fn move_up(&mut self) {
        match self.active_panel {
            Panel::Headers if !self.request.headers.is_empty() => {
                if self.selected_header > 0 {
                    self.selected_header -= 1;
                }
            }
            _ => {}
        }
    }

    fn move_down(&mut self) {
        match self.active_panel {
            Panel::Headers if !self.request.headers.is_empty() => {
                if self.selected_header < self.request.headers.len() - 1 {
                    self.selected_header += 1;
                }
            }
            _ => {}
        }
    }

    fn next_panel(&mut self) {
        self.active_panel = match self.active_panel {
            Panel::Url => Panel::QueryParams,
            Panel::QueryParams => Panel::Headers,
            Panel::Headers => Panel::Body,
            Panel::Body => Panel::Response,
            Panel::Response => Panel::Url,
        };
    }

    fn prev_panel(&mut self) {
        self.active_panel = match self.active_panel {
            Panel::Url => Panel::Response,
            Panel::QueryParams => Panel::Url,
            Panel::Headers => Panel::QueryParams,
            Panel::Body => Panel::Headers,
            Panel::Response => Panel::Body,
        };
    }

    fn handle_char_input(&mut self, c: char) {
        let req = InputRequest::InsertChar(c);
        match self.active_panel {
            Panel::Url => {
                self.request.url.handle(req);
            }
            Panel::Body => {
                self.request.body.handle(req);
            }
            _ => {}
        }
    }

    fn handle_backspace(&mut self) {
        let req = InputRequest::DeletePrevChar;
        match self.active_panel {
            Panel::Url => {
                self.request.url.handle(req);
            }
            Panel::Body => {
                self.request.body.handle(req);
            }
            _ => {}
        }
    }

    fn handle_enter_input(&mut self) {
        let req = InputRequest::InsertChar('\n');
        match self.active_panel {
            Panel::Url => {
                self.request.url.handle(req);
            }
            Panel::Body => {
                self.request.body.handle(req);
            }
            _ => {}
        }
    }

    fn handle_delete(&mut self) {
        let req = InputRequest::DeleteNextChar;
        match self.active_panel {
            Panel::Url => {
                self.request.url.handle(req);
            }
            Panel::Body => {
                self.request.body.handle(req);
            }
            _ => {}
        }
    }

    fn handle_cursor_left(&mut self) {
        let req = InputRequest::GoToPrevChar;
        match self.active_panel {
            Panel::Url => {
                self.request.url.handle(req);
            }
            Panel::Body => {
                self.request.body.handle(req);
            }
            _ => {}
        }
    }

    fn handle_cursor_right(&mut self) {
        let req = InputRequest::GoToNextChar;
        match self.active_panel {
            Panel::Url => {
                self.request.url.handle(req);
            }
            Panel::Body => {
                self.request.body.handle(req);
            }
            _ => {}
        }
    }

    fn handle_cursor_home(&mut self) {
        let req = InputRequest::GoToStart;
        match self.active_panel {
            Panel::Url => {
                self.request.url.handle(req);
            }
            Panel::Body => {
                self.request.body.handle(req);
            }
            _ => {}
        }
    }

    fn handle_cursor_end(&mut self) {
        let req = InputRequest::GoToEnd;
        match self.active_panel {
            Panel::Url => {
                self.request.url.handle(req);
            }
            Panel::Body => {
                self.request.body.handle(req);
            }
            _ => {}
        }
    }

    fn handle_query_param_char_input(&mut self, c: char) {
        if self.editing_header_key {
            self.new_query_param_key.push(c);
        } else {
            self.new_query_param_value.push(c);
        }
    }

    fn handle_header_char_input(&mut self, c: char) {
        if self.editing_header_key {
            self.new_header_key.push(c);
        } else {
            self.new_header_value.push(c);
        }
    }

    fn handle_query_param_backspace(&mut self) {
        if self.editing_query_param_key {
            self.new_query_param_key.pop();
        } else {
            self.new_query_param_value.pop();
        }
    }

    fn handle_header_backspace(&mut self) {
        if self.editing_header_key {
            self.new_header_key.pop();
        } else {
            self.new_header_value.pop();
        }
    }

    fn add_query_param(&mut self) {
        if !self.new_query_param_key.is_empty() {
            self.request.query_params.push((
                self.new_query_param_key.clone(),
                self.new_query_param_value.clone(),
            ));
            self.new_query_param_key.clear();
            self.new_query_param_value.clear();
            self.editing_header_key = true;
        }
    }

    fn add_header(&mut self) {
        if !self.new_header_key.is_empty() {
            self.request
                .headers
                .push((self.new_header_key.clone(), self.new_header_value.clone()));
            self.new_header_key.clear();
            self.new_header_value.clear();
            self.editing_header_key = true;
        }
    }

    fn delete_header(&mut self) {
        if !self.request.headers.is_empty() && self.selected_header < self.request.headers.len() {
            self.request.headers.remove(self.selected_header);
            if self.selected_header > 0 && self.selected_header >= self.request.headers.len() {
                self.selected_header = self.request.headers.len().saturating_sub(1);
            }
        }
    }

    fn delete_query_param(&mut self) {
        if !self.request.query_params.is_empty()
            && self.selected_query_param < self.request.query_params.len()
        {
            self.request.query_params.remove(self.selected_query_param);
            if self.selected_query_param > 0
                && self.selected_query_param >= self.request.query_params.len()
            {
                self.selected_query_param = self.request.query_params.len().saturating_sub(1);
            }
        }
    }

    fn send_request(&mut self) {
        self.is_loading = true;

        let request = self.request.clone();
        let method = self.request.method.clone();
        let url = self.request.url.to_string();
        let body = self.request.body.to_string();
        let headers = self.request.headers.clone();
        let query_params = self.request.query_params.clone();
        let tx = self.tx.clone();
        let his_tx = self.his_tx.clone();
        let error_tx = self.err_tx.clone();
        let mut http_client = self.http_client.clone();
        http_client.request_headers = headers;
        http_client.query_params = query_params;

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
                    let _ = error_tx.send(err.to_string());
                }
            }
        });
    }

    fn render(&self, frame: &mut Frame) {
        // Main vertical layout
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // URL bar
                Constraint::Min(10),   // Main content area
                Constraint::Length(3), // Status/history bar
            ])
            .split(frame.area());

        // URL Bar
        let method_color = match self.request.method {
            HttpMethod::GET => Color::Green,
            HttpMethod::POST => Color::Cyan,
            HttpMethod::PUT => Color::Yellow,
            HttpMethod::DELETE => Color::Red,
            HttpMethod::PATCH => Color::Magenta,
            HttpMethod::HEAD => Color::Cyan,
            HttpMethod::OPTIONS => Color::LightBlue,
        };

        let url_title = format!("{:?}", self.request.method);
        let url_style = if self.active_panel == Panel::Url && self.mode == Mode::Edit {
            Style::default().bg(Color::DarkGray)
        } else if self.active_panel == Panel::Url {
            Style::default().bg(Color::Cyan)
        } else {
            Style::default()
        };

        let url_display = self.request.url.to_string();
        let url_bar = Paragraph::new(Line::from(vec![
            Span::styled(
                format!("{} ", url_title),
                Style::default()
                    .fg(method_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(&url_display, url_style),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Request")
                .border_type(BorderType::Rounded),
        );
        frame.render_widget(url_bar, main_layout[0]);

        // Show cursor for URL field when in edit mode
        if self.mode == Mode::Edit && self.active_panel == Panel::Url {
            let layout = main_layout[1];
            let width = layout.width.saturating_sub(2); // Account for borders
            let scroll = self.request.url.visual_scroll(width as usize);
            // Position cursor accounting for method prefix and scroll
            let method_width = format!("{:?} ", self.request.method).len() as u16;
            let x = (self.request.url.visual_cursor().max(scroll) - scroll) as u16;
            frame.set_cursor_position((layout.x + 1 + method_width + x, layout.y + 1));
        }

        // Main content - split horizontally
        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Request panel
                Constraint::Percentage(50), // Response panel
            ])
            .split(main_layout[1]);

        // Request panel - split vertically
        let request_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8), // Query Params
                Constraint::Length(8), // Headers
                Constraint::Min(1),    // Body
            ])
            .split(content_layout[0]);

        // Headers panel
        self.render_query_params_panel(frame, request_layout[0]);
        self.render_headers_panel(frame, request_layout[1]);
        self.render_body_panel(frame, request_layout[2]);
        self.render_response_panel(frame, content_layout[1]);

        // Status bar
        let help_text = match (self.mode, &self.active_panel) {
            (Mode::Normal, Panel::Url) => {
                "hjkl: Navigate • i: Edit • m/M: Method • Enter: Send • q: Quit"
            }
            (Mode::Normal, Panel::Headers) => {
                "hjkl: Navigate • a: Add • d: Delete • i: Edit • q: Quit"
            }
            (Mode::Normal, Panel::Body) => "hjkl: Navigate • i: Edit • Enter: Send • q: Quit",
            (Mode::Normal, Panel::Response) => "hjkl: Navigate • Enter: Send • q: Quit",
            (Mode::Edit, Panel::Url) => "Esc: Normal • ←→: Move cursor • Home/End • Enter: Send",
            (Mode::Edit, Panel::Body) => {
                "Esc: Normal • ←→: Move cursor • Home/End • Enter: Newline"
            }
            (Mode::HeaderEdit, _) => "Tab: Next field • Enter: Save • Esc: Cancel",
            _ => "Esc: Normal mode",
        };

        let response_time = self.response.clone().unwrap_or_default().duration_ms;
        let color: Color = match response_time {
            0..=250 => Color::Green,
            251..700 => Color::Yellow,
            _ => Color::Red,
        };
        let status_bar = Paragraph::new(Line::from(vec![
            Span::styled("Response time: ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("{} ms", response_time), Style::default().fg(color)),
            Span::styled(" • Mode: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{:?}", self.mode), Style::default().fg(Color::Cyan)),
            Span::styled(" • ", Style::default().fg(Color::Gray)),
            Span::styled(help_text, Style::default().fg(Color::Gray)),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
        frame.render_widget(status_bar, main_layout[2]);

        // Temporary loading indicator
        if self.is_loading {
            let loading_area = Rect {
                x: frame.area().width / 2 - 16,
                y: frame.area().height / 2 - 3,
                width: 30,
                height: 5,
            };
            frame.render_widget(Clear, loading_area); // clear what's behind
            let loading_indicator = Paragraph::new("Please wait for the request to complete.")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title("Making request"),
                )
                .wrap(Wrap { trim: false });
            frame.render_widget(loading_indicator, loading_area);
        }

        // Error Box
        if let Some(error_msg) = &self.error {
            let area = Rect {
                x: frame.area().width / 2 - 20,
                y: frame.area().height / 2,
                width: 40,
                height: 5,
            };

            frame.render_widget(Clear, area);

            let error_box = Paragraph::new(error_msg.as_str())
                .wrap(ratatui::widgets::Wrap { trim: false })
                .style(Style::default().fg(Color::Red))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title("Error"),
                );

            frame.render_widget(error_box, area);
        }
    }

    fn render_query_params_panel(&self, frame: &mut Frame, area: ratatui::prelude::Rect) {
        let headers_style = if self.active_panel == Panel::QueryParams && self.mode == Mode::Normal
        {
            Style::default().fg(Color::White).bg(Color::Cyan)
        } else if self.active_panel == Panel::QueryParams {
            Style::default().fg(Color::White).bg(Color::DarkGray)
        } else {
            Style::default().fg(Color::Gray)
        };

        let mut query_param_items: Vec<ListItem> = self
            .request
            .query_params
            .iter()
            .enumerate()
            .map(|(i, (k, v))| {
                let style = if i == self.selected_header && self.active_panel == Panel::QueryParams
                {
                    Style::default().fg(Color::Black).bg(Color::Yellow)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}: {}", k, v)).style(style)
            })
            .collect();

        // Add new query param input if in header edit mode
        if self.mode == Mode::QueryParamEdit {
            let new_query_param_text = if self.editing_query_param_key {
                format!(
                    "→ {}: {}",
                    self.new_query_param_key, self.new_query_param_value
                )
            } else {
                format!(
                    "{}: → {}",
                    self.new_query_param_key, self.new_query_param_value
                )
            };
            query_param_items.push(
                ListItem::new(new_query_param_text).style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            );
        }

        let query_params = List::new(query_param_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Query Params"),
            )
            .style(headers_style);
        frame.render_widget(query_params, area);
    }

    fn render_headers_panel(&self, frame: &mut Frame, area: ratatui::prelude::Rect) {
        let headers_style = if self.active_panel == Panel::Headers && self.mode == Mode::Normal {
            Style::default().fg(Color::White).bg(Color::Cyan)
        } else if self.active_panel == Panel::Headers {
            Style::default().fg(Color::White).bg(Color::DarkGray)
        } else {
            Style::default().fg(Color::Gray)
        };

        let mut header_items: Vec<ListItem> = self
            .request
            .headers
            .iter()
            .enumerate()
            .map(|(i, (k, v))| {
                let style = if i == self.selected_header && self.active_panel == Panel::Headers {
                    Style::default().fg(Color::Black).bg(Color::Yellow)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}: {}", k, v)).style(style)
            })
            .collect();

        // Add new header input if in header edit mode
        if self.mode == Mode::HeaderEdit {
            let new_header_text = if self.editing_header_key {
                format!("→ {}: {}", self.new_header_key, self.new_header_value)
            } else {
                format!("{}: → {}", self.new_header_key, self.new_header_value)
            };
            header_items.push(
                ListItem::new(new_header_text).style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            );
        }

        let headers = List::new(header_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Headers"),
            )
            .style(headers_style);
        frame.render_widget(headers, area);
    }

    fn render_body_panel(&self, frame: &mut Frame, area: ratatui::prelude::Rect) {
        let body_style = if self.active_panel == Panel::Body && self.mode == Mode::Edit {
            Style::default().fg(Color::White).bg(Color::DarkGray)
        } else if self.active_panel == Panel::Body {
            Style::default().fg(Color::White).bg(Color::Cyan)
        } else {
            Style::default().fg(Color::Gray)
        };

        let body_display = self.request.body.to_string();
        let body = Paragraph::new(&*body_display)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Request Body"),
            )
            .style(body_style);
        frame.render_widget(body, area);

        // Show cursor for body field when in edit mode
        if self.mode == Mode::Edit && self.active_panel == Panel::Body {
            use unicode_segmentation::UnicodeSegmentation;

            let area_width = area.width.saturating_sub(2);
            let area_height = area.height.saturating_sub(2);

            let text = self.request.body.to_string();
            let cursor = self.request.body.visual_cursor();

            // Walk through lines to find line and column
            let mut remaining = cursor;
            let mut line_idx = 0;
            let mut col_idx = 0;

            for line in text.split('\n') {
                let len = line.graphemes(true).count();
                if remaining <= len {
                    col_idx = remaining;
                    break;
                } else {
                    remaining -= len + 1; // +1 for the '\n'
                    line_idx += 1;
                }
            }

            // Horizontal scroll based on current line only
            let line_text = text.split('\n').nth(line_idx).unwrap_or("");
            let line_len = line_text.graphemes(true).count();
            let max_scroll_x = line_len.saturating_sub(area_width as usize);
            let scroll_x = col_idx.min(max_scroll_x);

            // Vertical scroll based on current line
            let total_lines = text.lines().count();
            let max_scroll_y = total_lines.saturating_sub(area_height as usize);
            let scroll_y = line_idx.min(max_scroll_y);

            let x = (col_idx - scroll_x) as u16;
            let y = (line_idx - scroll_y) as u16;

            frame.set_cursor_position((area.x + 1 + x, area.y + 1 + y));
        }
    }

    fn render_response_panel(&self, frame: &mut Frame, area: ratatui::prelude::Rect) {
        let response_style = if self.active_panel == Panel::Response {
            Style::default().fg(Color::White).bg(Color::Cyan)
        } else {
            Style::default().fg(Color::Gray)
        };

        if let Some(ref response) = self.response {
            let response_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // Status line
                    Constraint::Length(15), // Response headers
                    Constraint::Min(1),    // Response body
                ])
                .split(area);

            // Status line
            let status_color = if response.status_code < 300 {
                Color::Green
            } else if response.status_code < 400 {
                Color::Yellow
            } else {
                Color::Red
            };

            let status =
                Paragraph::new(format!("{} {}", response.status_code, response.status_text)).style(
                    Style::default()
                        .fg(status_color)
                        .add_modifier(Modifier::BOLD),
                );
            frame.render_widget(status, response_layout[0]);

            // Response headers
            let resp_header_items: Vec<ListItem> = response
                .headers
                .iter()
                .map(|(k, v)| ListItem::new(format!("{}: {}", k, v)))
                .collect();

            let resp_headers = List::new(resp_header_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title("Response Headers"),
                )
                .style(Style::default().fg(Color::Gray));
            frame.render_widget(resp_headers, response_layout[1]);

            // Response body
            let resp_body = Paragraph::new(&*response.body)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title("Response Body"),
                )
                .style(response_style)
                .wrap(Wrap { trim: false });
            frame.render_widget(resp_body, response_layout[2]);
        } else {
            let empty_response = Paragraph::new("No response yet\n\nPress Enter to send request")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title("Response"),
                )
                .style(response_style)
                .alignment(Alignment::Center);
            frame.render_widget(empty_response, area);
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    loop {
        terminal.draw(|frame| app.render(frame))?;

        if crossterm::event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            app.handle_key(key.code, key.modifiers);
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

use std::{collections::HashMap, fmt::Display, time::Duration};

use ratatui::{
    Frame,
    crossterm::{
        self,
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    },
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
};
use tui_input::Input;
use tui_textarea::TextArea;

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

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_string = match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
        };

        write!(f, "{}", method_string)
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
struct App<'a> {
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
    url_input: TextArea<'a>,
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

impl<'a> App<'a> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let (err_tx, err_rx) = std::sync::mpsc::channel::<String>();
        let (his_tx, his_rx) = std::sync::mpsc::channel::<Request>();
        let http_client = http::HttpClient::default();
        let mut url_input = TextArea::default();
        url_input.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Enter the URL"),
        );

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
            url_input,
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        self.error = None;

        match self.mode {
            Mode::Normal => match key.code {
                KeyCode::Backspace => todo!(),
                KeyCode::Enter => self.send_request(),
                KeyCode::Left => todo!(),
                KeyCode::Right => todo!(),
                KeyCode::Up => todo!(),
                KeyCode::Down => todo!(),
                KeyCode::Home => todo!(),
                KeyCode::End => todo!(),
                KeyCode::PageUp => todo!(),
                KeyCode::PageDown => todo!(),
                KeyCode::Tab => match self.active_panel {
                    Panel::Url => self.active_panel = Panel::QueryParams,
                    Panel::QueryParams => self.active_panel = Panel::Headers,
                    Panel::Headers => self.active_panel = Panel::Body,
                    Panel::Body => self.active_panel = Panel::Response,
                    Panel::Response => self.active_panel = Panel::Url,
                },
                KeyCode::BackTab => match self.active_panel {
                    Panel::Url => self.active_panel = Panel::Response,
                    Panel::QueryParams => self.active_panel = Panel::Url,
                    Panel::Headers => self.active_panel = Panel::QueryParams,
                    Panel::Body => self.active_panel = Panel::Headers,
                    Panel::Response => self.active_panel = Panel::Body,
                },
                KeyCode::Delete => todo!(),
                KeyCode::Insert => todo!(),
                KeyCode::F(_) => todo!(),
                KeyCode::Char('i') => self.mode = Mode::Edit,
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Char('m') => match self.request.method {
                    HttpMethod::GET => self.request.method = HttpMethod::POST,
                    HttpMethod::POST => self.request.method = HttpMethod::PUT,
                    HttpMethod::PUT => self.request.method = HttpMethod::DELETE,
                    HttpMethod::DELETE => self.request.method = HttpMethod::PATCH,
                    HttpMethod::PATCH => self.request.method = HttpMethod::HEAD,
                    HttpMethod::HEAD => self.request.method = HttpMethod::OPTIONS,
                    HttpMethod::OPTIONS => self.request.method = HttpMethod::GET,
                },
                KeyCode::Null => todo!(),
                KeyCode::Esc => todo!(),
                KeyCode::CapsLock => todo!(),
                KeyCode::ScrollLock => todo!(),
                KeyCode::NumLock => todo!(),
                KeyCode::PrintScreen => todo!(),
                KeyCode::Pause => todo!(),
                KeyCode::Menu => todo!(),
                KeyCode::KeypadBegin => todo!(),
                _ => {}
            },
            Mode::Edit => match self.active_panel {
                Panel::Url => match key.code {
                    KeyCode::Esc => self.mode = Mode::Normal,
                    KeyCode::Enter => self.send_request(),
                    _ => {
                        self.url_input.input(key);
                    }
                },
                Panel::QueryParams => todo!(),
                Panel::Headers => todo!(),
                Panel::Body => todo!(),
                Panel::Response => todo!(),
            },
            Mode::HeaderEdit => todo!(),
            Mode::QueryParamEdit => todo!(),
        }
    }

    fn send_request(&mut self) {
        self.is_loading = true;

        let request = self.request.clone();
        let method = self.request.method.clone();
        let url = self.url_input.lines().join("\n");
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

        // URL input layout
        let url_input_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(10), Constraint::Min(5)])
            .split(main_layout[0]);

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

        let method = Paragraph::new(self.request.method.to_string())
            .style(Style::new().fg(method_color))
            .block(Block::new().borders(Borders::ALL).title("Method"));

        frame.render_widget(method, url_input_layout[0]);
        frame.render_widget(&self.url_input, url_input_layout[1]);

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
                    Constraint::Length(1),  // Status line
                    Constraint::Length(15), // Response headers
                    Constraint::Min(1),     // Response body
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

impl<'a> Default for App<'a> {
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
            app.handle_key(key);
            // app.url_input.input(key);
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

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::ui::{
    App,
    types::{HttpMethod, Mode, Panel},
};

pub(crate) fn render(app: &mut App, frame: &mut Frame) {
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
    let method_color = match app.request.method {
        HttpMethod::GET => Color::Green,
        HttpMethod::POST => Color::Cyan,
        HttpMethod::PUT => Color::Yellow,
        HttpMethod::DELETE => Color::Red,
        HttpMethod::PATCH => Color::Magenta,
        HttpMethod::HEAD => Color::Cyan,
        HttpMethod::OPTIONS => Color::LightBlue,
    };

    let method = Paragraph::new(app.request.method.to_string())
        .style(Style::new().fg(method_color))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Method"),
        );

    let url_bg_color = match app.active_panel {
        Panel::Url => Color::Cyan,
        _ => Color::Reset,
    };

    let headers_style = if app.active_panel == Panel::Url && app.mode == Mode::Normal {
        Style::default().fg(Color::White).bg(Color::Cyan)
    } else if app.active_panel == Panel::Url {
        Style::default().fg(Color::White).bg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Gray)
    };

    app.url_input
        .set_placeholder_text("https://jsonplaceholder.typicode.com/posts");
    app.url_input.set_block(
        Block::default()
            .fg(url_bg_color)
            .borders(Borders::all())
            .border_type(BorderType::Rounded)
            .title("URL")
            .style(headers_style),
    );
    frame.render_widget(method, url_input_layout[0]);
    frame.render_widget(&app.url_input, url_input_layout[1]);

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
    render_query_params_panel(app, frame, request_layout[0]);
    render_headers_panel(app, frame, request_layout[1]);
    render_body_panel(app, frame, request_layout[2]);
    render_response_panel(app, frame, content_layout[1]);

    // Status bar
    let help_text = match (app.mode, &app.active_panel) {
        (Mode::Normal, Panel::Url) => {
            "hjkl: Navigate • i: Edit • m/M: Method • Enter: Send • q: Quit"
        }
        (Mode::Normal, Panel::Headers) => "hjkl: Navigate • a: Add • d: Delete • i: Edit • q: Quit",
        (Mode::Normal, Panel::Body) => "hjkl: Navigate • i: Edit • Enter: Send • q: Quit",
        (Mode::Normal, Panel::Response) => "hjkl: Navigate • Enter: Send • q: Quit",
        (Mode::Edit, Panel::Url) => "Esc: Normal • ←→: Move cursor • Home/End • Enter: Send",
        (Mode::Edit, Panel::Body) => "Esc: Normal • ←→: Move cursor • Home/End • Enter: Newline",
        _ => "Esc: Normal mode",
    };

    let response_time = app.response.clone().unwrap_or_default().duration_ms;
    let color: Color = match response_time {
        0..=250 => Color::Green,
        251..700 => Color::Yellow,
        _ => Color::Red,
    };
    let status_bar = Paragraph::new(Line::from(vec![
        Span::styled("Mode: ", Style::default().fg(Color::Gray)),
        Span::styled(format!("{} ", app.mode), Style::default().fg(Color::Cyan)),
        Span::styled("Response time: ", Style::default().fg(Color::Cyan)),
        Span::styled(format!("{} ms", response_time), Style::default().fg(color)),
        Span::styled(" • ", Style::default().fg(Color::Gray)),
        Span::styled(help_text, Style::default().fg(Color::Gray)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );
    frame.render_widget(status_bar, main_layout[2]);

    if app.edit_modal {
        // A modal to add key: val data
        let modal_area = Rect {
            x: frame.area().width / 2 - 16,
            y: frame.area().height / 2 - 3,
            width: 30,
            height: 5,
        };
        frame.render_widget(Clear, modal_area);
    }

    // Temporary loading indicator
    if app.is_loading {
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
    if let Some(error_msg) = &app.error {
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

fn render_query_params_panel(app: &mut App, frame: &mut Frame, area: ratatui::prelude::Rect) {
    let headers_style = if app.active_panel == Panel::QueryParams && app.mode == Mode::Normal {
        Style::default().fg(Color::White).bg(Color::Cyan)
    } else if app.active_panel == Panel::QueryParams {
        Style::default().fg(Color::White).bg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Gray)
    };

    app.query_params_input.set_block(
        Block::default()
            .title("Query Params")
            .borders(Borders::all())
            .border_type(BorderType::Rounded)
            .style(headers_style),
    );
    app.query_params_input
        .set_line_number_style(Style::default());
    frame.render_widget(&app.query_params_input, area);
}

fn render_headers_panel(app: &mut App, frame: &mut Frame, area: ratatui::prelude::Rect) {
    let headers_style = if app.active_panel == Panel::Headers && app.mode == Mode::Normal {
        Style::default().fg(Color::White).bg(Color::Cyan)
    } else if app.active_panel == Panel::Headers {
        Style::default().fg(Color::White).bg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Gray)
    };

    app.headers_input.set_block(
        Block::default()
            .title("Request Headers")
            .borders(Borders::all())
            .border_type(BorderType::Rounded)
            .style(headers_style),
    );
    app.headers_input.set_line_number_style(Style::default());
    frame.render_widget(&app.headers_input, area);
}

fn render_body_panel(app: &App, frame: &mut Frame, area: ratatui::prelude::Rect) {
    let body_style = if app.active_panel == Panel::Body && app.mode == Mode::Edit {
        Style::default().fg(Color::White).bg(Color::DarkGray)
    } else if app.active_panel == Panel::Body {
        Style::default().fg(Color::White).bg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    };

    let body_display = app.request.body.to_string();
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
    if app.mode == Mode::Edit && app.active_panel == Panel::Body {
        use unicode_segmentation::UnicodeSegmentation;

        let area_width = area.width.saturating_sub(2);
        let area_height = area.height.saturating_sub(2);

        let text = app.request.body.to_string();
        let cursor = app.request.body.visual_cursor();

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

fn render_response_panel(app: &App, frame: &mut Frame, area: ratatui::prelude::Rect) {
    let response_style = if app.active_panel == Panel::Response {
        Style::default().fg(Color::White).bg(Color::Cyan)
    } else {
        Style::default().fg(Color::Gray)
    };

    if let Some(ref response) = app.response {
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

        let status = Paragraph::new(format!("{} {}", response.status_code, response.status_text))
            .style(
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
            .wrap(Wrap { trim: false })
            .scroll((app.response_scroll, 0));
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

pub mod keyboard;
pub mod layout;
pub mod sections;

use std::{env, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use crate::ui::sections::{
    method::{Method, MethodProps},
    query_params::{QueryParams, QueryParamsProps},
    request_body::{RequestBody, RequestBodyProps},
    request_headers::{RequestHeaders, RequestHeadersProps},
    response_body::ResponseBody,
    response_headers::{ResponseHeaders, ResponseHeadersProps},
    status_bar::{StatusBar, StatusBarProps},
    url_bar::UrlBar,
};
use crate::{
    types::app::{ActivePanel, App, Mode},
    ui::layout::MainLayout,
};

impl<'a> App<'a> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();

            if event::poll(Duration::from_millis(50)).unwrap() {
                let event = event::read().unwrap();
                if let event::Event::Key(key_event) = event {
                    self.handle_key_events(key_event)
                }
            }

            self.poll_network();

            if self.app_state.should_exit {
                break;
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let l = MainLayout::split(frame.area());
        let active_panel = self.app_state.active_panel;

        // Method box
        Method.render(
            frame,
            MethodProps {
                area: l.method,
                active: active_panel == ActivePanel::Url,
                label: self.method_label(),
            },
        );

        // URL bar
        let url_bar = UrlBar(self);
        url_bar.render(frame, l.url);

        // Request sections (left)
        QueryParams.render(
            frame,
            QueryParamsProps {
                area: l.req_query,
                active: active_panel == ActivePanel::ReqQuery,
                value: self.req_query_input.value(),
                cursor: self.req_query_input.cursor(),
                show_cursor: self.app_state.mode == Mode::Edit
                    && active_panel == ActivePanel::ReqQuery,
            },
        );

        RequestHeaders.render(
            frame,
            RequestHeadersProps {
                area: l.req_headers,
                active: active_panel == ActivePanel::ReqHeaders,
                value: self.req_headers_input.value(),
                cursor: self.req_headers_input.cursor(),
                show_cursor: self.app_state.mode == Mode::Edit
                    && active_panel == ActivePanel::ReqHeaders,
            },
        );

        RequestBody.render(
            frame,
            RequestBodyProps {
                area: l.req_body,
                active: active_panel == ActivePanel::ReqBody,
                value: self.req_body_input.value(),
                cursor: self.req_body_input.cursor(),
                show_cursor: self.app_state.mode == Mode::Edit
                    && active_panel == ActivePanel::ReqBody,
                content_type: self.body_content_type(),
            },
        );

        // Response sections (right)
        self.app_state.response_viewport_height = l.res_body.height.saturating_sub(2);
        self.app_state.response_viewport_width = l.res_body.width.saturating_sub(2);
        let max_scroll = self
            .app_state
            .response_line_count
            .saturating_sub(self.app_state.response_viewport_height as usize);
        let max_scroll = (max_scroll.min(u16::MAX as usize)) as u16;
        if self.app_state.response_scroll > max_scroll {
            self.app_state.response_scroll = max_scroll;
        }
        let max_scroll_x = self
            .app_state
            .response_max_line_width
            .saturating_sub(self.app_state.response_viewport_width as usize);
        let max_scroll_x = (max_scroll_x.min(u16::MAX as usize)) as u16;
        if self.app_state.response_scroll_x > max_scroll_x {
            self.app_state.response_scroll_x = max_scroll_x;
        }

        ResponseBody.render(
            frame,
            l.res_body,
            active_panel == ActivePanel::ResBody,
            self.app_state.response_body.as_deref(),
            self.app_state.response_formatted_body.as_deref(),
            self.app_state.response_content_type.as_deref(),
            self.app_state.response_scroll,
            self.app_state.response_scroll_x,
        );

        ResponseHeaders.render(
            frame,
            ResponseHeadersProps {
                area: l.res_headers,
                active: active_panel == ActivePanel::ResHeaders,
                status: self.app_state.response_status.as_deref(),
                headers: self.app_state.response_headers.as_deref(),
                response_time: self.app_state.response_time,
            },
        );

        // Status bar (bottom)
        StatusBar.render(
            frame,
            StatusBarProps {
                area: l.status,
                mode: self.app_state.mode,
                active_panel,
                is_loading: self.app_state.is_loading,
                error: self.app_state.error.as_deref(),
            },
        );

        if self.app_state.is_loading {
            let loading_area = centered_area(frame.area(), 42, 7);
            frame.render_widget(Clear, loading_area);
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

        if let Some(error_msg) = &self.app_state.error {
            let error_area = centered_area(frame.area(), 40, 5);
            frame.render_widget(Clear, error_area);
            let error_box = Paragraph::new(error_msg.as_str())
                .wrap(Wrap { trim: false })
                .style(Style::default().fg(Color::Red))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title("Error"),
                );
            frame.render_widget(error_box, error_area);
        }
    }
}

pub fn run() {
    let mut application = App::default();

    if let Some(url) = env::args().nth(1) {
        application = application.with_default_url(&url);
    }
    ratatui::run(|terminal| application.run(terminal))
}

fn centered_area(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width.saturating_sub(2));
    let height = height.min(area.height.saturating_sub(2));
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect {
        x,
        y,
        width,
        height,
    }
}

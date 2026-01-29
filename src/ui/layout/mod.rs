use ratatui::layout::{Constraint, Direction, Layout, Rect};

#[derive(Debug, Clone, Copy)]
pub struct MainLayout {
    pub method: Rect,
    pub url: Rect,

    pub req_query: Rect,
    pub req_headers: Rect,
    pub req_body: Rect,

    pub res_headers: Rect,
    pub res_body: Rect,

    pub status: Rect,
}

impl MainLayout {
    /// Split the full screen into:
    /// - Top bar (method + url)
    /// - Main content split (request left, response right)
    /// - Bottom status bar
    pub fn split(area: Rect) -> Self {
        // Whole screen: header, content, status
        let [header, content, status] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // method/url bar height
                Constraint::Min(1),
                Constraint::Length(1), // status line
            ])
            .split(area)
            .as_ref()
            .try_into()
            .expect("main vertical split must yield 3 rects");

        // Header: method box + url bar
        let [method, url] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(12), // "Method" box width
                Constraint::Min(1),
            ])
            .split(header)
            .as_ref()
            .try_into()
            .expect("header split must yield 2 rects");

        // Content: request left + response right
        let [request, response] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(content)
            .as_ref()
            .try_into()
            .expect("content split must yield 2 rects");

        // Request pane: query, headers, body (stacked)
        let [req_query, req_headers, req_body] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(request)
            .as_ref()
            .try_into()
            .expect("request split must yield 3 rects");

        // Response pane: headers + body
        let [res_headers, res_body] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
            .split(response)
            .as_ref()
            .try_into()
            .expect("response split must yield 2 rects");

        Self {
            method,
            url,
            req_query,
            req_headers,
            req_body,
            res_headers,
            res_body,
            status,
        }
    }
}

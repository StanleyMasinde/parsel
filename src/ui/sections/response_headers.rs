use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub struct ResponseHeaders;

impl ResponseHeaders {
    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        active: bool,
        status: Option<&str>,
        headers: Option<&str>,
    ) {
        let title = if active {
            "● Response Headers"
        } else {
            "○ Response Headers"
        };
        let border_style = if active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let mut content = String::new();
        if let Some(status) = status {
            content.push_str("Status: ");
            content.push_str(status);
        }
        if let Some(headers) = headers {
            if !content.is_empty() {
                content.push('\n');
            }
            content.push_str(headers);
        }
        frame.render_widget(
            Paragraph::new(content).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title(title),
            ),
            area,
        );
    }
}

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct RequestHeaders;

impl RequestHeaders {
    pub fn render(&self, frame: &mut Frame, area: Rect, active: bool) {
        let title = if active {
            "● Request Headers"
        } else {
            "○ Request Headers"
        };
        let border_style = if active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        frame.render_widget(
            Paragraph::new("accept: application/json\nauthorization: Bearer <token>")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(border_style)
                        .title(title),
                )
                .wrap(Wrap { trim: true }),
            area,
        );
    }
}

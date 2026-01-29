use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct RequestHeaders;

impl RequestHeaders {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("accept: application/json\nauthorization: Bearer <token>")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Request Headers"),
                )
                .wrap(Wrap { trim: true }),
            area,
        );
    }
}

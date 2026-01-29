use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

pub struct ResponseHeaders;

impl ResponseHeaders {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("").block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Response Headers"),
            ),
            area,
        );
    }
}

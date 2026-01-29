use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct RequestBody;

impl RequestBody {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("{\n  \"hello\": \"world\"\n}")
                .block(Block::default().borders(Borders::ALL).title("Request Body"))
                .wrap(Wrap { trim: true }),
            area,
        );
    }
}

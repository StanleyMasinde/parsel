use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

pub struct Method;

impl Method {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("GET")
                .block(Block::default().borders(Borders::ALL).title("Method"))
                .centered(),
            area,
        );
    }
}

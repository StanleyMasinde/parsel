use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct QueryParams;

impl QueryParams {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new(Line::from("id: 42"))
                .block(Block::default().borders(Borders::ALL).title("Query Params"))
                .wrap(Wrap { trim: true }),
            area,
        );
    }
}

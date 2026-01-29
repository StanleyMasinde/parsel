use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct UrlBar;

impl UrlBar {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let widget = Paragraph::new("https://api.example.com/users")
            .block(Block::default().borders(Borders::ALL).title("URL"))
            .wrap(Wrap { trim: true });

        frame.render_widget(widget, area);
    }
}

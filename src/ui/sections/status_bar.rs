use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Paragraph},
};

pub struct StatusBar;

impl StatusBar {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new(
                "NORMAL • Focus: URL • i: Edit • Enter: Send • h: History • : Command  RT: -",
            )
            .block(Block::default()),
            area,
        );
    }
}

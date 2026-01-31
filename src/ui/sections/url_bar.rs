use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::types::app::{ActivePanel, App, Mode};

pub struct UrlBar<'a>(pub &'a App);

impl<'a> UrlBar<'a> {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let widget = Paragraph::new(self.0.url_input.value())
            .block(Block::default().borders(Borders::ALL).title("URL"))
            .wrap(Wrap { trim: true });

        frame.render_widget(widget, area);

        if self.0.app_state.mode == Mode::Edit && self.0.app_state.active_panel == ActivePanel::Url
        {
            let cursor = self.0.url_input.visual_cursor() as u16;
            frame.set_cursor_position((area.x + cursor + 1, area.y + 1));
        }
    }
}

use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::types::app::{ActivePanel, App, Mode};

pub struct UrlBar<'a>(pub &'a App<'a>);

impl<'a> UrlBar<'a> {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let active = self.0.app_state.active_panel == ActivePanel::Url;
        let title = if active { "● URL" } else { "○ URL" };
        let border_style = if active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let widget = Paragraph::new(self.0.url_input.value())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title(title),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(widget, area);

        if self.0.app_state.mode == Mode::Edit && active {
            let max_col = area.width.saturating_sub(2);
            let cursor = self.0.url_input.visual_cursor() as u16;
            let cursor = cursor.min(max_col);
            frame.set_cursor_position((area.x + cursor + 1, area.y + 1));
        }
    }
}

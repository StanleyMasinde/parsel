use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct QueryParams;

impl QueryParams {
    pub fn render(&self, frame: &mut Frame, area: Rect, active: bool, value: &str) {
        let title = if active {
            "● Query Params"
        } else {
            "○ Query Params"
        };
        let border_style = if active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };
        let content = if value.is_empty() {
            "key: val"
        } else {
            value
        };

        frame.render_widget(
            Paragraph::new(content)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(border_style)
                        .title(title),
                )
                .wrap(Wrap { trim: false }),
            area,
        );
    }
}

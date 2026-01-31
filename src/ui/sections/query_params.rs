use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct QueryParams;

impl QueryParams {
    pub fn render(&self, frame: &mut Frame, area: Rect, active: bool) {
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

        frame.render_widget(
            Paragraph::new(Line::from("id: 42"))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(border_style)
                        .title(title),
                )
                .wrap(Wrap { trim: true }),
            area,
        );
    }
}

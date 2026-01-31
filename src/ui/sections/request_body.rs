use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct RequestBody;

impl RequestBody {
    pub fn render(&self, frame: &mut Frame, area: Rect, active: bool) {
        let title = if active {
            "● Request Body"
        } else {
            "○ Request Body"
        };
        let border_style = if active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        frame.render_widget(
            Paragraph::new("{\n  \"hello\": \"world\"\n}")
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

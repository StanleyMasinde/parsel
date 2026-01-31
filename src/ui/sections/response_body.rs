use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub struct ResponseBody;

impl ResponseBody {
    pub fn render(&self, frame: &mut Frame, area: Rect, active: bool) {
        let title = if active { "● Response" } else { "○ Response" };
        let border_style = if active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        frame.render_widget(
            Paragraph::new("No response yet\n\nPress Enter to send request")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(border_style)
                        .title(title),
                )
                .centered(),
            area,
        );
    }
}

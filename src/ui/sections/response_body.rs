use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

pub struct ResponseBody;

impl ResponseBody {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("No response yet\n\nPress Enter to send request")
                .block(Block::default().borders(Borders::ALL).title("Response"))
                .centered(),
            area,
        );
    }
}

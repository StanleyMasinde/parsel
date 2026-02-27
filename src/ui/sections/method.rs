use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub struct Method;

pub struct MethodProps<'a> {
    pub area: Rect,
    pub active: bool,
    pub label: &'a str,
}

impl Method {
    pub fn render(&self, frame: &mut Frame, props: MethodProps<'_>) {
        let MethodProps {
            area,
            active,
            label,
        } = props;
        let title = if active { "● Method" } else { "○ Method" };
        let border_style = if active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        frame.render_widget(
            Paragraph::new(label)
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

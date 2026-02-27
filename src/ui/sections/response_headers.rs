use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub struct ResponseHeaders;

pub struct ResponseHeadersProps<'a> {
    pub area: Rect,
    pub active: bool,
    pub status: Option<&'a str>,
    pub headers: Option<&'a str>,
    pub response_time: u128,
}

impl ResponseHeaders {
    pub fn render(&self, frame: &mut Frame, props: ResponseHeadersProps<'_>) {
        let ResponseHeadersProps {
            area,
            active,
            status,
            headers,
            response_time,
        } = props;
        let mut title_text: String = "Response headers".to_string();

        if response_time > 0 {
            title_text = format!("Response headers | {}ms ", response_time);
        }

        let title = if active {
            format!("● {title_text}")
        } else {
            format!("○ {title_text}")
        };

        let border_style = if active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let mut content = String::new();
        if let Some(status) = status {
            content.push_str("Status: ");
            content.push_str(status);
        }
        if let Some(headers) = headers {
            if !content.is_empty() {
                content.push('\n');
            }
            content.push_str(headers);
        }
        frame.render_widget(
            Paragraph::new(content).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title(title),
            ),
            area,
        );
    }
}

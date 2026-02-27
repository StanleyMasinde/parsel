use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
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
        let indicator = if active { "● " } else { "○ " };
        let mut title_spans = vec![
            Span::raw(indicator),
            Span::raw("Response headers"),
        ];

        if response_time > 0 {
            let response_time_color = response_time_color(response_time);
            title_spans.push(Span::raw(" | "));
            title_spans.push(Span::styled(
                format!("{response_time}ms"),
                Style::default().fg(response_time_color),
            ));
        }

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
                    .title(Line::from(title_spans)),
            ),
            area,
        );
    }
}

fn response_time_color(response_time: u128) -> Color {
    if response_time <= 300 {
        Color::Green
    } else if response_time <= 1000 {
        Color::Rgb(255, 191, 0)
    } else {
        Color::Red
    }
}

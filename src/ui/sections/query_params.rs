use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct QueryParams;

impl QueryParams {
    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        active: bool,
        value: &str,
        cursor: usize,
        show_cursor: bool,
    ) {
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
        let content = if value.is_empty() { "key: val" } else { value };

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

        if show_cursor {
            let (line, col) = cursor_position(value, cursor, area.width.saturating_sub(2));
            if area.height > 2 {
                let line = line.min(area.height.saturating_sub(2) as usize);
                let col = col.min(area.width.saturating_sub(2) as usize);
                frame.set_cursor_position((area.x + col as u16 + 1, area.y + line as u16 + 1));
            }
        }
    }
}

fn cursor_position(value: &str, cursor: usize, width: u16) -> (usize, usize) {
    if width == 0 {
        return (0, 0);
    }
    let width = width as usize;
    let mut line = 0usize;
    let mut col = 0usize;
    for (idx, ch) in value.chars().enumerate() {
        if idx >= cursor {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
            continue;
        }
        col += 1;
        if col >= width {
            line += 1;
            col = 0;
        }
    }
    (line, col)
}

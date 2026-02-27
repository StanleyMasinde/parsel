use once_cell::sync::Lazy;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

pub struct ResponseBody;

static HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "embedded",
    "function",
    "function.builtin",
    "keyword",
    "module",
    "number",
    "operator",
    "property",
    "property.builtin",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "boolean",
    "null",
];

static JSON_HIGHLIGHT: Lazy<HighlightConfiguration> = Lazy::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter_json::LANGUAGE.into(),
        "json",
        tree_sitter_json::HIGHLIGHTS_QUERY,
        "",
        "",
    )
    .expect("tree-sitter json highlight config");
    config.configure(HIGHLIGHT_NAMES);
    config
});

static HTML_HIGHLIGHT: Lazy<HighlightConfiguration> = Lazy::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter_html::LANGUAGE.into(),
        "html",
        tree_sitter_html::HIGHLIGHTS_QUERY,
        tree_sitter_html::INJECTIONS_QUERY,
        "",
    )
    .expect("tree-sitter html highlight config");
    config.configure(HIGHLIGHT_NAMES);
    config
});

impl ResponseBody {
    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        active: bool,
        body: Option<&str>,
        formatted_body: Option<&str>,
        content_type: Option<&str>,
        scroll: u16,
        scroll_x: u16,
    ) {
        let title = if active {
            "● Response"
        } else {
            "○ Response"
        };
        let border_style = if active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let content = display_content(body, formatted_body, content_type);
        let text = if body.is_some() {
            highlight_body(&content, content_type).unwrap_or_else(|| Text::from(content))
        } else {
            Text::from(content)
        };
        frame.render_widget(
            Paragraph::new(text).scroll((scroll, scroll_x)).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title(title),
            ),
            area,
        );
    }

    pub fn line_count(
        &self,
        body: Option<&str>,
        formatted_body: Option<&str>,
        content_type: Option<&str>,
    ) -> usize {
        let content = display_content(body, formatted_body, content_type);
        let count = content.lines().count();
        if count == 0 { 1 } else { count }
    }

    pub fn max_line_width(
        &self,
        body: Option<&str>,
        formatted_body: Option<&str>,
        content_type: Option<&str>,
        max_line_width_cache: Option<usize>,
    ) -> usize {
        max_line_width_cache.unwrap_or_else(|| {
            let content = display_content(body, formatted_body, content_type);
            content
                .lines()
                .map(|line| line.chars().count())
                .max()
                .unwrap_or(1)
        })
    }
}

pub fn format_for_display(body: Option<&str>, content_type: Option<&str>) -> Option<String> {
    let body = body?;
    format_body(body, content_type)
}

fn display_content<'a>(
    body: Option<&'a str>,
    formatted_body: Option<&'a str>,
    content_type: Option<&str>,
) -> std::borrow::Cow<'a, str> {
    if let Some(formatted) = formatted_body {
        return std::borrow::Cow::Borrowed(formatted);
    }

    let content = body.unwrap_or("No response yet\n\nPress Enter to send request");
    if body.is_some() {
        if let Some(formatted) = format_body(content, content_type) {
            return std::borrow::Cow::Owned(formatted);
        }
    }

    std::borrow::Cow::Borrowed(content)
}

fn highlight_body(body: &str, content_type: Option<&str>) -> Option<Text<'static>> {
    let config = content_type.and_then(map_content_type_to_highlight_config)?;
    let mut highlighter = Highlighter::new();
    let events = highlighter
        .highlight(config, body.as_bytes(), None, |_| None)
        .ok()?;
    let mut lines: Vec<Line> = Vec::new();
    let mut current: Vec<Span> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();

    for event in events {
        match event.ok()? {
            HighlightEvent::HighlightStart(id) => stack.push(id.0),
            HighlightEvent::HighlightEnd => {
                stack.pop();
            }
            HighlightEvent::Source { start, end } => {
                let style = stack
                    .last()
                    .and_then(|id| HIGHLIGHT_NAMES.get(*id))
                    .map(|name| style_for_highlight_name(name))
                    .unwrap_or_default();
                let slice = &body[start..end];
                for (idx, piece) in slice.split('\n').enumerate() {
                    if idx > 0 {
                        lines.push(Line::from(std::mem::take(&mut current)));
                    }
                    if !piece.is_empty() {
                        current.push(Span::styled(piece.to_string(), style));
                    }
                }
            }
        }
    }

    lines.push(Line::from(current));
    Some(Text::from(lines))
}

fn format_body(body: &str, content_type: Option<&str>) -> Option<String> {
    let normalized = content_type
        .and_then(|ct| ct.split(';').next())
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    match normalized.as_str() {
        "application/json" | "text/json" | "application/ld+json" => format_json(body),
        "text/html" => format_html(body),
        _ => None,
    }
}

fn format_json(body: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(body).ok()?;
    serde_json::to_string_pretty(&value).ok()
}

fn format_html(body: &str) -> Option<String> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    let mut in_tag = false;
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    for ch in body.chars() {
        if ch == '<' {
            if !buf.is_empty() {
                let text = buf.trim();
                if !text.is_empty() {
                    tokens.push(text.to_string());
                }
                buf.clear();
            }
            in_tag = true;
            buf.push(ch);
            continue;
        }
        if in_tag {
            if ch == '\'' && !in_double_quote {
                in_single_quote = !in_single_quote;
            } else if ch == '"' && !in_single_quote {
                in_double_quote = !in_double_quote;
            }
        }
        if ch == '>' && in_tag && !in_single_quote && !in_double_quote {
            buf.push(ch);
            tokens.push(buf.clone());
            buf.clear();
            in_tag = false;
            in_single_quote = false;
            in_double_quote = false;
            continue;
        }
        buf.push(ch);
    }
    if !buf.is_empty() {
        let text = buf.trim();
        if !text.is_empty() {
            tokens.push(text.to_string());
        }
    }

    let mut lines = Vec::new();
    let mut indent = 0usize;
    for token in tokens {
        if token.starts_with('<') && token.ends_with('>') {
            let tag = token.trim();
            let is_comment = tag.starts_with("<!--");
            let is_doctype = tag.starts_with("<!");
            let is_pi = tag.starts_with("<?");
            let is_closing = tag.starts_with("</");
            let is_self_closing =
                tag.ends_with("/>") || is_comment || is_doctype || is_pi || is_void_element(tag);

            if is_closing {
                indent = indent.saturating_sub(1);
            }

            lines.push(format!("{}{}", "  ".repeat(indent), tag));

            if !is_closing && !is_self_closing {
                indent += 1;
            }
        } else {
            lines.push(format!("{}{}", "  ".repeat(indent), token));
        }
    }

    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}

fn is_void_element(tag: &str) -> bool {
    let name = tag
        .trim_start_matches('<')
        .trim_start_matches('/')
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_end_matches('>')
        .trim_end_matches('/');
    matches!(
        name.to_ascii_lowercase().as_str(),
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

fn map_content_type_to_highlight_config(
    content_type: &str,
) -> Option<&'static HighlightConfiguration> {
    let normalized = content_type
        .split(';')
        .next()
        .unwrap_or(content_type)
        .trim()
        .to_ascii_lowercase();
    match normalized.as_str() {
        "application/json" | "text/json" | "application/ld+json" => Some(&JSON_HIGHLIGHT),
        "text/html" => Some(&HTML_HIGHLIGHT),
        _ => None,
    }
}

fn style_for_highlight_name(name: &str) -> Style {
    match name {
        "string" | "string.special" => Style::default().fg(Color::Green),
        "number" => Style::default().fg(Color::Yellow),
        "boolean" | "constant" | "constant.builtin" | "null" => Style::default().fg(Color::Cyan),
        "property" | "property.builtin" | "attribute" => Style::default().fg(Color::Blue),
        "tag" | "tag.builtin" | "keyword" => Style::default().fg(Color::Magenta),
        "comment" => Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
        "punctuation" | "punctuation.bracket" | "punctuation.delimiter" | "punctuation.special" => {
            Style::default().fg(Color::DarkGray)
        }
        "type" | "type.builtin" | "constructor" => Style::default().fg(Color::LightBlue),
        _ => Style::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::format_html;

    #[test]
    fn format_html_ignores_gt_inside_double_quotes() {
        let input = r#"<div data-text="a > b"><span>ok</span></div>"#;
        let expected = [
            r#"<div data-text="a > b">"#,
            "  <span>",
            "    ok",
            "  </span>",
            "</div>",
        ]
        .join("\n");

        assert_eq!(format_html(input).as_deref(), Some(expected.as_str()));
    }

    #[test]
    fn format_html_ignores_gt_inside_single_quotes() {
        let input = "<a title='x > y'>link</a>";
        let expected = ["<a title='x > y'>", "  link", "</a>"].join("\n");

        assert_eq!(format_html(input).as_deref(), Some(expected.as_str()));
    }
}

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Paragraph},
};

use crate::types::app::{ActivePanel, Mode};

pub struct StatusBar;

pub struct StatusBarProps<'a> {
    pub area: Rect,
    pub mode: Mode,
    pub active_panel: ActivePanel,
    pub is_loading: bool,
    pub error: Option<&'a str>,
}

impl StatusBar {
    pub fn render(&self, frame: &mut Frame, props: StatusBarProps<'_>) {
        let StatusBarProps {
            area,
            mode,
            active_panel,
            is_loading,
            error,
        } = props;
        let mode_label = match mode {
            Mode::Normal => "NORMAL",
            Mode::Edit => "EDIT",
        };
        let focus_label = match active_panel {
            ActivePanel::Url => "URL",
            ActivePanel::ReqQuery => "Query",
            ActivePanel::ReqHeaders => "ReqHeaders",
            ActivePanel::ReqBody => "ReqBody",
            ActivePanel::ResHeaders => "ResHeaders",
            ActivePanel::ResBody => "ResBody",
        };
        let hint = match mode {
            Mode::Normal => {
                "i: Edit • Enter: Send • b/B: Body • Tab/Shift+Tab: Focus • h/l: ResBody X-Scroll"
            }
            Mode::Edit => "Esc: Normal • Enter: Send (URL) • Ctrl+Enter: Send",
        };
        let status = if let Some(error) = error {
            format!("ERROR: {}", error)
        } else if is_loading {
            "Loading...".to_string()
        } else {
            "Ready".to_string()
        };

        frame.render_widget(
            Paragraph::new(format!(
                "{mode} • Focus: {focus} • {hint} • {status}",
                mode = mode_label,
                focus = focus_label,
                hint = hint,
                status = status
            ))
            .block(Block::default()),
            area,
        );
    }
}

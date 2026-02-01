use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Paragraph},
};

use crate::types::app::{ActivePanel, Mode};

pub struct StatusBar;

impl StatusBar {
    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        mode: Mode,
        active_panel: ActivePanel,
        is_loading: bool,
        error: Option<&str>,
    ) {
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
            Mode::Normal => "i: Edit • Enter: Send • b/B: Body • Tab/Shift+Tab: Focus",
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

use ratatui::{
    crossterm::event::KeyEvent,
    widgets::{Paragraph, Widget},
};
use tui_input::Input;

use crate::types::input_handler::{InputHandler, InputState};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Normal,
    Edit,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ActivePanel {
    #[default]
    Url,
}

impl ActivePanel {
    pub fn next(self) -> Self {
        Self::Url
    }

    pub fn prev(self) -> Self {
        Self::Url
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub should_exit: bool,
    pub mode: Mode,
    pub active_panel: ActivePanel,
}

#[derive(Debug, Default)]
pub struct App {
    pub app_state: AppState,
    pub url_input: Input,
}
impl App {
    pub(crate) fn handle_key_events(&mut self, key_event: KeyEvent) {
        let mut input_handler = InputHandler::new(self, InputState::default());
        input_handler.handle(key_event);
    }
}

impl<'app> Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new("Parsel").centered().render(area, buf);
    }
}

use ratatui::crossterm::event::KeyCode;

use crate::types::app::App;

pub struct InputState {
    pub key_code: KeyCode,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            key_code: KeyCode::Null,
        }
    }
}

pub struct InputHandler<'a> {
    pub app: &'a mut App,
    pub state: InputState,
}

impl<'a> InputHandler<'a> {
    pub fn new(app: &'a mut App, state: InputState) -> Self {
        Self { app, state }
    }
}

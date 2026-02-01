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

pub struct InputHandler<'b, 'a> {
    pub app: &'b mut App<'a>,
    pub state: InputState,
}

impl<'b, 'a> InputHandler<'b, 'a> {
    pub fn new(app: &'b mut App<'a>, state: InputState) -> Self {
        Self { app, state }
    }
}

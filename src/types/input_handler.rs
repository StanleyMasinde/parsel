use ratatui::crossterm::event::KeyCode;

use crate::types::app::App;

pub struct InputState {
    pub key_code: KeyCode,
}

pub struct InputHandler<'input> {
    pub app: App<'input>,
    pub state: InputState,
}

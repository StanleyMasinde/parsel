use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::types::app::Mode;
use crate::types::input_handler::InputHandler;

impl<'handler> InputHandler<'handler> {
    pub fn handle(&mut self, key: KeyEvent) {
        match self.app.app_state.mode {
            Mode::Normal => {
                self.state.key_code = key.code;
                self.normal_mode();
            }
            Mode::Edit => {
                self.state.key_code = key.code;
                self.edit_mode()
            }
        }
    }

    fn normal_mode(&self) {
        match self.state.key_code {
            KeyCode::Backspace => todo!(),
            KeyCode::Enter => todo!(),
            KeyCode::Left => todo!(),
            KeyCode::Right => todo!(),
            KeyCode::Up => todo!(),
            KeyCode::Down => todo!(),
            KeyCode::Home => todo!(),
            KeyCode::End => todo!(),
            KeyCode::PageUp => todo!(),
            KeyCode::PageDown => todo!(),
            KeyCode::Tab => todo!(),
            KeyCode::BackTab => todo!(),
            KeyCode::Delete => todo!(),
            KeyCode::Insert => todo!(),
            KeyCode::F(_) => todo!(),
            KeyCode::Char(_) => todo!(),
            KeyCode::Null => todo!(),
            KeyCode::Esc => todo!(),
            KeyCode::CapsLock => todo!(),
            KeyCode::ScrollLock => todo!(),
            KeyCode::NumLock => todo!(),
            KeyCode::PrintScreen => todo!(),
            KeyCode::Pause => todo!(),
            KeyCode::Menu => todo!(),
            KeyCode::KeypadBegin => todo!(),
            KeyCode::Media(_media_key_code) => {
                todo!()
            }
            KeyCode::Modifier(_modifier_key_code) => {
                todo!()
            }
        }
    }

    fn edit_mode(&self) {
        todo!()
    }
}

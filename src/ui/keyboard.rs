use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use tui_input::backend::crossterm::EventHandler;

use crate::types::app::{ActivePanel, Mode};
use crate::types::input_handler::InputHandler;

impl<'a> InputHandler<'a> {
    pub fn handle(&mut self, key: KeyEvent) {
        match self.app.app_state.mode {
            Mode::Normal => {
                self.state.key_code = key.code;
                self.normal_mode();
            }
            Mode::Edit => {
                self.state.key_code = key.code;
                self.edit_mode(key)
            }
        }
    }

    fn normal_mode(&mut self) {
        if self.app.app_state.active_panel != ActivePanel::Url {
            todo!(
                "unhandled normal input for non-url panel: {:?}",
                self.app.app_state.active_panel
            );
        }

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
            KeyCode::Tab => {
                self.app.app_state.active_panel = self.app.app_state.active_panel.next();
            }
            KeyCode::BackTab => {
                self.app.app_state.active_panel = self.app.app_state.active_panel.prev();
            }
            KeyCode::Delete => todo!(),
            KeyCode::Insert => todo!(),
            KeyCode::F(_) => todo!(),
            KeyCode::Char('i') => {
                self.app.app_state.mode = Mode::Edit;
            }
            KeyCode::Char(_) => todo!(),
            KeyCode::Null => todo!(),
            KeyCode::Esc => {}
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

    fn edit_mode(&mut self, key: KeyEvent) {
        if self.app.app_state.active_panel != ActivePanel::Url {
            todo!(
                "unhandled edit input for non-url panel: {:?}",
                self.app.app_state.active_panel
            );
        }

        match self.state.key_code {
            KeyCode::Char(_)
            | KeyCode::Backspace
            | KeyCode::Delete
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::Home
            | KeyCode::End => {
                self.app.url_input.handle_event(&Event::Key(key));
            }
            KeyCode::Esc => {
                self.app.app_state.mode = Mode::Normal;
            }
            KeyCode::Enter => todo!(),
            KeyCode::Up => todo!(),
            KeyCode::Down => todo!(),
            KeyCode::PageUp => todo!(),
            KeyCode::PageDown => todo!(),
            KeyCode::Tab => todo!(),
            KeyCode::BackTab => todo!(),
            KeyCode::Insert => todo!(),
            KeyCode::F(_) => todo!(),
            KeyCode::Null => todo!(),
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
}

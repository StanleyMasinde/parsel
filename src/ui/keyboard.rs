use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::{InputRequest, backend::crossterm::EventHandler};

use crate::types::app::{ActivePanel, Mode};
use crate::types::input_handler::InputHandler;

impl<'b, 'a> InputHandler<'b, 'a> {
    pub fn handle(&mut self, key: KeyEvent) {
        if self.app.app_state.error.is_some() {
            self.app.app_state.error = None;
            return;
        }

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
        match self.state.key_code {
            KeyCode::Backspace => {}
            KeyCode::Enter => self.app.send_request(),
            KeyCode::Left => {}
            KeyCode::Right => {}
            KeyCode::Up => {}
            KeyCode::Down => {}
            KeyCode::Home => {}
            KeyCode::End => {}
            KeyCode::PageUp => {}
            KeyCode::PageDown => {}
            KeyCode::Tab | KeyCode::Char('j') | KeyCode::Char('l') => {
                self.app.app_state.active_panel = self.app.app_state.active_panel.next();
            }
            KeyCode::BackTab | KeyCode::Char('k') | KeyCode::Char('h') => {
                self.app.app_state.active_panel = self.app.app_state.active_panel.prev();
            }
            KeyCode::Delete => {}
            KeyCode::Insert => {}
            KeyCode::F(_) => {}
            KeyCode::Char('i') => {
                if matches!(
                    self.app.app_state.active_panel,
                    ActivePanel::Url
                        | ActivePanel::ReqQuery
                        | ActivePanel::ReqHeaders
                        | ActivePanel::ReqBody
                ) {
                    self.app.app_state.mode = Mode::Edit;
                }
            }
            KeyCode::Char('m') => {
                self.app.next_method();
            }
            KeyCode::Char('M') => {
                self.app.prev_method();
            }
            KeyCode::Char('b') => {
                self.app.next_body_mode();
            }
            KeyCode::Char('B') => {
                self.app.prev_body_mode();
            }
            KeyCode::Char('q') => {
                self.app.app_state.should_exit = true;
            }
            KeyCode::Char(_) => {}
            KeyCode::Null => {}
            KeyCode::Esc => {}
            KeyCode::CapsLock => {}
            KeyCode::ScrollLock => {}
            KeyCode::NumLock => {}
            KeyCode::PrintScreen => {}
            KeyCode::Pause => {}
            KeyCode::Menu => {}
            KeyCode::KeypadBegin => {}
            KeyCode::Media(_media_key_code) => {}
            KeyCode::Modifier(_modifier_key_code) => {}
        }
    }

    fn edit_mode(&mut self, key: KeyEvent) {
        let active_panel = self.app.app_state.active_panel;
        let active_input = match active_panel {
            ActivePanel::Url => Some(&mut self.app.url_input),
            ActivePanel::ReqQuery => Some(&mut self.app.req_query_input),
            ActivePanel::ReqHeaders => Some(&mut self.app.req_headers_input),
            ActivePanel::ReqBody => Some(&mut self.app.req_body_input),
            _ => None,
        };

        match self.state.key_code {
            KeyCode::Char(_)
            | KeyCode::Backspace
            | KeyCode::Delete
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::Home
            | KeyCode::End => {
                if let Some(active_input) = active_input {
                    active_input.handle_event(&Event::Key(key));
                }
            }
            KeyCode::Esc => {
                self.app.app_state.mode = Mode::Normal;
            }
            KeyCode::Enter => {
                if active_panel == ActivePanel::Url || key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    self.app.send_request();
                } else if let Some(active_input) = active_input {
                    active_input.handle(InputRequest::InsertChar('\n'));
                }
            }
            KeyCode::Up => {}
            KeyCode::Down => {}
            KeyCode::PageUp => {}
            KeyCode::PageDown => {}
            KeyCode::Tab => {}
            KeyCode::BackTab => {}
            KeyCode::Insert => {}
            KeyCode::F(_) => {}
            KeyCode::Null => {}
            KeyCode::CapsLock => {}
            KeyCode::ScrollLock => {}
            KeyCode::NumLock => {}
            KeyCode::PrintScreen => {}
            KeyCode::Pause => {}
            KeyCode::Menu => {}
            KeyCode::KeypadBegin => {}
            KeyCode::Media(_media_key_code) => {}
            KeyCode::Modifier(_modifier_key_code) => {}
        }
    }
}

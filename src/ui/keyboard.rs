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
            KeyCode::Backspace => todo!(),
            KeyCode::Enter => self.app.send_request(),
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
                if matches!(
                    self.app.app_state.active_panel,
                    ActivePanel::Url | ActivePanel::ReqQuery | ActivePanel::ReqHeaders
                ) {
                    self.app.app_state.mode = Mode::Edit;
                } else {
                    todo!(
                        "unhandled normal input for non-url panel: {:?}",
                        self.app.app_state.active_panel
                    );
                }
            }
            KeyCode::Char('j') => {
                if self.app.app_state.active_panel == ActivePanel::ResBody {
                    let max_scroll = self
                        .app
                        .app_state
                        .response_line_count
                        .saturating_sub(self.app.app_state.response_viewport_height as usize);
                    let max_scroll = (max_scroll.min(u16::MAX as usize)) as u16;
                    if self.app.app_state.response_scroll < max_scroll {
                        self.app.app_state.response_scroll += 1;
                    }
                }
            }
            KeyCode::Char('k') => {
                if self.app.app_state.active_panel == ActivePanel::ResBody {
                    self.app.app_state.response_scroll =
                        self.app.app_state.response_scroll.saturating_sub(1);
                }
            }
            KeyCode::Char('m') => {
                self.app.next_method();
            }
            KeyCode::Char('M') => {
                self.app.prev_method();
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
        let active_panel = self.app.app_state.active_panel;
        let active_input = match active_panel {
            ActivePanel::Url => Some(&mut self.app.url_input),
            ActivePanel::ReqQuery => Some(&mut self.app.req_query_input),
            ActivePanel::ReqHeaders => Some(&mut self.app.req_headers_input),
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
                } else {
                    todo!("unhandled edit input for panel: {:?}", active_panel);
                }
            }
            KeyCode::Esc => {
                self.app.app_state.mode = Mode::Normal;
            }
            KeyCode::Enter => {
                if active_panel == ActivePanel::Url
                    || key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    self.app.send_request();
                } else if let Some(active_input) = active_input {
                    active_input.handle(InputRequest::InsertChar('\n'));
                } else {
                    todo!("unhandled edit input for panel: {:?}", active_panel);
                }
            }
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

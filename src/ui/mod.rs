pub mod keyboard;
pub mod layout;
pub mod sections;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyEvent},
};

use crate::ui::sections::{
    method::Method, query_params::QueryParams, request_body::RequestBody,
    request_headers::RequestHeaders, response_body::ResponseBody,
    response_headers::ResponseHeaders, status_bar::StatusBar, url_bar::UrlBar,
};
use crate::{types::app::App, ui::layout::MainLayout};

impl<'app> App<'app> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();

            match event::read().unwrap() {
                event::Event::FocusGained => todo!(),
                event::Event::FocusLost => todo!(),
                event::Event::Key(key_event) => self.handle_events(key_event),
                event::Event::Mouse(_mouse_event) => todo!(),
                event::Event::Paste(_) => todo!(),
                event::Event::Resize(_, _) => todo!(),
            }
            if self.app_state.should_exit {
                break;
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let l = MainLayout::split(frame.area());

        // Method box
        Method.render(frame, l.method);

        // URL bar
        UrlBar.render(frame, l.url);

        // Request sections (left)
        QueryParams.render(frame, l.req_query);

        RequestHeaders.render(frame, l.req_headers);

        RequestBody.render(frame, l.req_body);

        // Response sections (right)
        ResponseBody.render(frame, l.res_body);

        ResponseHeaders.render(frame, l.res_headers);

        // Status bar (bottom)
        StatusBar.render(frame, l.status);
    }

    fn handle_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            event::KeyCode::Backspace => todo!(),
            event::KeyCode::Enter => todo!(),
            event::KeyCode::Left => todo!(),
            event::KeyCode::Right => todo!(),
            event::KeyCode::Up => todo!(),
            event::KeyCode::Down => todo!(),
            event::KeyCode::Home => todo!(),
            event::KeyCode::End => todo!(),
            event::KeyCode::PageUp => todo!(),
            event::KeyCode::PageDown => todo!(),
            event::KeyCode::Tab => todo!(),
            event::KeyCode::BackTab => todo!(),
            event::KeyCode::Delete => todo!(),
            event::KeyCode::Insert => todo!(),
            event::KeyCode::F(_) => todo!(),
            event::KeyCode::Char(_) => todo!(),
            event::KeyCode::Null => todo!(),
            event::KeyCode::Esc => todo!(),
            event::KeyCode::CapsLock => todo!(),
            event::KeyCode::ScrollLock => todo!(),
            event::KeyCode::NumLock => todo!(),
            event::KeyCode::PrintScreen => todo!(),
            event::KeyCode::Pause => todo!(),
            event::KeyCode::Menu => todo!(),
            event::KeyCode::KeypadBegin => todo!(),
            event::KeyCode::Media(_media_key_code) => todo!(),
            event::KeyCode::Modifier(_modifier_key_code) => todo!(),
        }
    }
}
pub fn run() {
    ratatui::run(|terminal| App::default().run(terminal))
}

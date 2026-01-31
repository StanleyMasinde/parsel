pub mod keyboard;
pub mod layout;
pub mod sections;

use ratatui::{DefaultTerminal, Frame, crossterm::event};

use crate::ui::sections::{
    method::Method, query_params::QueryParams, request_body::RequestBody,
    request_headers::RequestHeaders, response_body::ResponseBody,
    response_headers::ResponseHeaders, status_bar::StatusBar, url_bar::UrlBar,
};
use crate::{types::app::App, ui::layout::MainLayout};

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        loop {
            terminal.draw(|frame| self.draw(frame)).unwrap();

            let event = event::read().unwrap();
            match event {
                event::Event::FocusGained => todo!(),
                event::Event::FocusLost => todo!(),
                event::Event::Key(key_event) => self.handle_key_events(key_event),
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
        let url_bar = UrlBar(self);
        url_bar.render(frame, l.url);

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
}
pub fn run() {
    ratatui::run(|terminal| App::default().run(terminal))
}

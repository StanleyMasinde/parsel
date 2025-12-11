use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::ui::{
    App,
    types::{HttpMethod, Mode, Panel},
};

pub(crate) fn handle_key(app: &mut App, key: KeyEvent) {
    app.error = None;

    match app.mode {
        Mode::Normal => match key.code {
            KeyCode::Enter => app.send_request(),
            KeyCode::Tab => match app.active_panel {
                Panel::Url => app.active_panel = Panel::QueryParams,
                Panel::QueryParams => app.active_panel = Panel::Headers,
                Panel::Headers => app.active_panel = Panel::Body,
                Panel::Body => app.active_panel = Panel::Response,
                Panel::Response => app.active_panel = Panel::Url,
            },
            KeyCode::BackTab => match app.active_panel {
                Panel::Url => app.active_panel = Panel::Response,
                Panel::QueryParams => app.active_panel = Panel::Url,
                Panel::Headers => app.active_panel = Panel::QueryParams,
                Panel::Body => app.active_panel = Panel::Headers,
                Panel::Response => app.active_panel = Panel::Body,
            },
            KeyCode::Char('j') => match app.active_panel {
                Panel::Url => app.active_panel = Panel::QueryParams,
                Panel::QueryParams => app.active_panel = Panel::Headers,
                Panel::Headers => app.active_panel = Panel::Body,
                Panel::Body => app.active_panel = Panel::Response,
                Panel::Response => {
                    app.response_scroll += 10;
                    app.active_panel = Panel::Response
                }
            },
            KeyCode::Char('k') if app.active_panel == Panel::Response => {
                if app.response_scroll > 0 {
                    app.response_scroll -= 10;
                }
            }
            KeyCode::Char('k') => {}
            KeyCode::Char('i') => app.mode = Mode::Edit,
            KeyCode::Char('q') => app.should_quit = true,
            KeyCode::Char('m') => match app.request.method {
                HttpMethod::GET => app.request.method = HttpMethod::POST,
                HttpMethod::POST => app.request.method = HttpMethod::PUT,
                HttpMethod::PUT => app.request.method = HttpMethod::DELETE,
                HttpMethod::DELETE => app.request.method = HttpMethod::PATCH,
                HttpMethod::PATCH => app.request.method = HttpMethod::HEAD,
                HttpMethod::HEAD => app.request.method = HttpMethod::OPTIONS,
                HttpMethod::OPTIONS => app.request.method = HttpMethod::GET,
            },
            _ => {
                panic!("All keys need to be handled!")
            }
        },
        Mode::Edit => match app.active_panel {
            Panel::Url => match key.code {
                KeyCode::Esc => app.mode = Mode::Normal,
                KeyCode::Enter => app.send_request(),
                _ => {
                    app.url_input.input(key);
                }
            },
            Panel::QueryParams => match key.code {
                KeyCode::Esc => app.mode = Mode::Normal,
                KeyCode::Tab => app.query_params_input.insert_char(':'),
                _ => {
                    app.query_params_input.input(key);
                }
            },
            _ => {
                panic!("All keys need to be handled!")
            }
        },
    }
}

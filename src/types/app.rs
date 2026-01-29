use ratatui::widgets::{Paragraph, Widget};
use tui_textarea::TextArea;

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Normal,
    Edit,
}

#[derive(Debug, Default)]
pub struct AppState {
    pub should_exit: bool,
    pub mode: Mode,
}

#[derive(Debug, Default)]
pub struct App<'app> {
    pub app_state: AppState,
    pub url_input: TextArea<'app>,
}

impl<'app> Widget for &App<'app> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new("Parsel").centered().render(area, buf);
    }
}

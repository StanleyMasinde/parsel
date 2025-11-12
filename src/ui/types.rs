use std::{collections::HashMap, fmt::Display};

use tui_input::Input;


#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_string = match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
        };

        write!(f, "{}", method_string)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Request {
    pub(crate) method: HttpMethod,
    pub(crate) headers: Vec<(String, String)>,
    pub(crate) body: Input,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Response {
    pub(crate) status_code: u16,
    pub(crate) status_text: String,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: String,
    pub(crate) duration_ms: u128,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Panel {
    Url,
    QueryParams,
    Headers,
    Body,
    Response,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Mode {
    Normal,
    Edit,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            Mode::Normal => "NORMAL",
            Mode::Edit => "EDIT",
        };

        write!(f, "{}", mode)
    }
}

#[derive(Debug)]
pub(crate) enum InputField {
    Key,
    Value,
}

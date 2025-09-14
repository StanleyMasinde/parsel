use std::{collections::HashMap, time::Instant};

use reqwest::{
    StatusCode,
    header::{HeaderMap, HeaderName, HeaderValue},
};

pub struct HttpResponse {
    pub status: StatusCode,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub elapsed: u128,
}

fn vec_to_headermap(pairs: Vec<(String, String)>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    for (k, v) in pairs {
        let name = HeaderName::from_bytes(k.as_bytes()).unwrap_or_else(|_| HeaderName::from_static("Fooo"));
        let value = HeaderValue::from_str(&v).unwrap_or_else(|_| HeaderValue::from_static("Bar"));
        headers.insert(name, value);
    }
    headers
}

fn headers_to_map(headers: &HeaderMap) -> HashMap<String, String> {
    headers
        .iter()
        .map(|(k, v)| {
            let key = k.to_string();
            let val = v.to_str().unwrap_or("Foo").to_string();
            (key, val)
        })
        .collect()
}

pub fn get(
    url: &str,
    request_headers: Vec<(String, String)>,
) -> Result<HttpResponse, reqwest::Error> {
    let start = Instant::now();
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .headers(vec_to_headermap(request_headers))
        .send()?;
    let elapsed = start.elapsed().as_millis();
    let status = response.status();
    let headers = headers_to_map(response.headers());
    let text = response.text()?;

    let status_text = match status {
        StatusCode::OK => "OK".to_string(),
        StatusCode::CREATED => "Created".to_string(),
        StatusCode::NO_CONTENT => "No Content".to_string(),
        _ => "Unknown".to_string(),
    };

    let res = HttpResponse {
        status,
        body: text,
        status_text,
        headers,
        elapsed,
    };

    Ok(res)
}

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
        let name = HeaderName::from_bytes(k.as_bytes())
            .unwrap_or_else(|_| HeaderName::from_static("Fooo"));
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

pub trait RestClient {
    fn get(&self, path: &str) -> Result<HttpResponse, reqwest::Error>;
    fn post(
        &self,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<HttpResponse, reqwest::Error>;
    fn put(
        &self,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<HttpResponse, reqwest::Error>;
    fn patch(
        &self,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<HttpResponse, reqwest::Error>;
    fn delete(&self, path: &str) -> Result<HttpResponse, reqwest::Error>;
}

#[derive(Debug, Default, Clone)]
pub struct HttpClient {
    pub request_headers: Vec<(String, String)>,
    pub query_params: Vec<(String, String)>,
}

impl HttpClient {
    fn get_status_text(&self, status: StatusCode) -> String {
        match status {
            StatusCode::OK => "OK".to_string(),
            StatusCode::CREATED => "Created".to_string(),
            StatusCode::NO_CONTENT => "No Content".to_string(),
            StatusCode::BAD_REQUEST => "Bad Request".to_string(),
            StatusCode::UNAUTHORIZED => "Unauthorized".to_string(),
            StatusCode::FORBIDDEN => "Forbidden".to_string(),
            StatusCode::NOT_FOUND => "Not found".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

impl RestClient for HttpClient {
    fn get(&self, url: &str) -> Result<HttpResponse, reqwest::Error> {
        let request_headers = &self.request_headers.to_vec();
        let start = Instant::now();
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(url)
            .query(&self.query_params)
            .headers(vec_to_headermap(request_headers.to_vec()))
            .send()?;

        let elapsed = start.elapsed().as_millis();
        let status = response.status();
        let headers = headers_to_map(response.headers());
        let text = response.text()?;

        let status_text = self.get_status_text(status);

        let res = HttpResponse {
            status,
            body: text,
            status_text,
            headers,
            elapsed,
        };

        Ok(res)
    }

    fn post(
        &self,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<HttpResponse, reqwest::Error> {
        let request_headers = &self.request_headers.to_vec();
        let start = Instant::now();
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(path)
            .headers(vec_to_headermap(request_headers.to_vec()))
            .json(&body)
            .send()?;
        let elapsed = start.elapsed().as_millis();
        let status = response.status();
        let headers = headers_to_map(response.headers());
        let text = response.text()?;

        let status_text = self.get_status_text(status);

        let res = HttpResponse {
            status,
            body: text,
            status_text,
            headers,
            elapsed,
        };

        Ok(res)
    }

    fn put(
        &self,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<HttpResponse, reqwest::Error> {
        let request_headers = &self.request_headers.to_vec();
        let start = Instant::now();
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(path)
            .query(&self.query_params)
            .headers(vec_to_headermap(request_headers.to_vec()))
            .json(&body)
            .send()?;
        let elapsed = start.elapsed().as_millis();
        let status = response.status();
        let headers = headers_to_map(response.headers());
        let text = response.text()?;

        let status_text = self.get_status_text(status);

        let res = HttpResponse {
            status,
            body: text,
            status_text,
            headers,
            elapsed,
        };

        Ok(res)
    }

    fn patch(
        &self,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<HttpResponse, reqwest::Error> {
        let request_headers = &self.request_headers.to_vec();
        let start = Instant::now();
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(path)
            .query(&self.query_params)
            .headers(vec_to_headermap(request_headers.to_vec()))
            .json(&body)
            .send()?;
        let elapsed = start.elapsed().as_millis();
        let status = response.status();
        let headers = headers_to_map(response.headers());
        let text = response.text()?;

        let status_text = self.get_status_text(status);

        let res = HttpResponse {
            status,
            body: text,
            status_text,
            headers,
            elapsed,
        };

        Ok(res)
    }

    fn delete(&self, path: &str) -> Result<HttpResponse, reqwest::Error> {
        let request_headers = &self.request_headers.to_vec();
        let start = Instant::now();
        let client = reqwest::blocking::Client::new();
        let response = client
            .delete(path)
            .query(&self.query_params)
            .headers(vec_to_headermap(request_headers.to_vec()))
            .send()?;

        let elapsed = start.elapsed().as_millis();
        let status = response.status();
        let headers = headers_to_map(response.headers());
        let text = response.text()?;

        let status_text = self.get_status_text(status);
        let res = HttpResponse {
            status,
            body: text,
            status_text,
            headers,
            elapsed,
        };

        Ok(res)
    }
}

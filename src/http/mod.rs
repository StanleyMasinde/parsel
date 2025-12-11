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

fn vec_to_headermap(pairs: Vec<String>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    pairs.iter().for_each(|line| {
        let parts = line.splitn(2, ":").collect::<Vec<&str>>();
        let key = parts.iter().nth(0);
        let val = parts.iter().nth(1);

        if key.is_some() {
            let name = HeaderName::from_bytes(key.unwrap().as_bytes()).unwrap();
            let val = HeaderValue::from_str(val.unwrap_or(&"").trim()).unwrap();

            headers.insert(name, val);
        }
    });
    headers
}

fn vec_to_query_params<'a>(params: Vec<String>) -> Vec<(String, String)> {
    params
        .iter()
        .filter_map(|line| {
            let mut parts = line.splitn(2, ':');
            let key = parts.next()?.trim().to_string();
            let value = parts.next().unwrap_or("").trim().to_string();
            if key.is_empty() {
                None
            } else {
                Some((key, value))
            }
        })
        .collect()
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
    pub request_headers: HeaderMap,
    pub query_params: Vec<(String, String)>,
}

impl HttpClient {
    fn get_status_text(&self, status: StatusCode) -> &str {
        match status {
            StatusCode::OK => "OK",
            StatusCode::CREATED => "Created",
            StatusCode::NO_CONTENT => "No Content",
            StatusCode::BAD_REQUEST => "Bad Request",
            StatusCode::UNAUTHORIZED => "Unauthorized",
            StatusCode::FORBIDDEN => "Forbidden",
            StatusCode::NOT_FOUND => "Not found",
            _ => "Unknown",
        }
    }

    pub fn with_query_params(mut self, query_params: Vec<String>) -> Self {
        self.query_params = vec_to_query_params(query_params);
        self
    }

    pub fn with_request_headers(mut self, headers: Vec<String>) -> Self {
        self.request_headers = vec_to_headermap(headers);
        self
    }
}

impl RestClient for HttpClient {
    fn get(&self, url: &str) -> Result<HttpResponse, reqwest::Error> {
        let start = Instant::now();
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(url)
            .query(&self.query_params)
            .headers(self.request_headers.clone())
            .send()?;

        let elapsed = start.elapsed().as_millis();
        let status = response.status();
        let headers = headers_to_map(response.headers());
        let text = response.text()?;

        let status_text = self.get_status_text(status);

        let res = HttpResponse {
            status,
            body: text,
            status_text: status_text.to_string(),
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
        let start = Instant::now();
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(path)
            .headers(self.request_headers.clone())
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
            status_text: status_text.to_string(),
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
        let start = Instant::now();
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(path)
            .query(&self.query_params)
            .headers(self.request_headers.clone())
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
            status_text: status_text.to_string(),
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
        let start = Instant::now();
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(path)
            .query(&self.query_params)
            .headers(self.request_headers.clone())
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
            status_text: status_text.to_string(),
            headers,
            elapsed,
        };

        Ok(res)
    }

    fn delete(&self, path: &str) -> Result<HttpResponse, reqwest::Error> {
        let start = Instant::now();
        let client = reqwest::blocking::Client::new();
        let response = client
            .delete(path)
            .query(&self.query_params)
            .headers(self.request_headers.clone())
            .send()?;

        let elapsed = start.elapsed().as_millis();
        let status = response.status();
        let headers = headers_to_map(response.headers());
        let text = response.text()?;

        let status_text = self.get_status_text(status);
        let res = HttpResponse {
            status,
            body: text,
            status_text: status_text.to_string(),
            headers,
            elapsed,
        };

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_to_header_map() {
        let pairs = vec![
            "Accept: application/json".to_string(),
            "Accept-Language: en-US,en;q=0.5.to".to_string(),
        ];
        let header_map = vec_to_headermap(pairs);

        assert_eq!(header_map.get("Accept").unwrap(), "application/json");
    }
}

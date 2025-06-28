use std::collections::HashMap;
use std::io::{Result, Write};


#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse {
    version: String, 
    status_code: String, 
    status_text: String, 
    headers: Option<HashMap<String, String>>, 
    resp_body: Option<String>
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            version: String::from("HTTP/1.1"),
            status_code: String::from("200"),
            status_text: String::from("OK"),
            headers: None,
            resp_body: None,
        }
    }
}

impl From<HttpResponse> for String {
    fn from(resp: HttpResponse) -> String {
        let headers_string = match &resp.headers {
            Some(headers) => {
                headers
                    .iter()
                    .map(|(key, value)| format!("{}: {}\r\n", key, value))
                    .collect::<String>()
            },
            None => String::new()
        };
        let (content_length, body) = match &resp.resp_body {
            Some(body_content) => (body_content.len(), body_content.as_str()),
            None => (0, ""),
        };
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &resp.version,      // 使用借用引用
            &resp.status_code,  // 使用借用引用
            &resp.status_text,  // 使用借用引用
            headers_string,
            content_length,
            body
        )
    }
}

impl HttpResponse {
    pub fn new(
        status_code: &str, 
        headers: Option<HashMap<String, String>>, 
        resp_body: Option<String>
    ) -> Self {
        let mut resp = HttpResponse::default();

        resp.status_code = status_code.to_string();
        resp.status_text = match status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "403" => "Forbidden",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Unknown"
        }.to_string();
        resp.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "text/html".to_string());
                Some(h)
            }
        };
        resp.resp_body = resp_body;

        resp
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()>{
        let response = self.clone();
        let response_string = String::from(response);
        write!(write_stream, "{}", response_string)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("xxxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1".to_string(),
            status_code: "200".to_string(),
            status_text: "OK".to_string(),
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "text/html".to_string());
                Some(h)
            },
            resp_body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new("404", None, Some("xxxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1".to_string(),
            status_code: "404".to_string(),
            status_text: "Not Found".to_string(),
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "text/html".to_string());
                Some(h)
            },
            resp_body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1".to_string(),
            status_code: "404".to_string(),
            status_text: "Not Found".to_string(),
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "text/html".to_string());
                Some(h)
            },
            resp_body: Some("xxxx".into()),
        };
        let http_string: String = response_expected.into();
        let actual_string =
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: 4\r\n\r\nxxxx"
                .to_string();
        assert_eq!(http_string, actual_string);
    }
}

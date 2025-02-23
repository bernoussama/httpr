use std::collections::HashMap;

pub struct Response {
    pub status_line: StatusLine,
    pub headers: HashMap<String, Vec<String>>,
    pub body: Vec<u8>,
}

pub struct StatusLine {
    pub version: String,
    pub status_code: u16,
    pub reason_phrase: Option<String>,
}

impl StatusLine {
    pub fn to_bytes(&self) -> Vec<u8> {
        let reason_phrase = self.reason_phrase.clone().unwrap_or("".to_string());
        format!("{} {} {}", self.version, self.status_code, reason_phrase)
            .as_bytes()
            .to_vec()
    }
}

impl Response {
    pub fn new() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Server".to_string(), vec!["faras".to_string()]);
        headers.insert("Content-Type".to_string(), vec!["text/plain".to_string()]);
        Response {
            status_line: StatusLine {
                version: "HTTP/1.1".to_string(),
                status_code: 200,
                reason_phrase: Some("OK".to_string()),
            },
            body: Vec::new(),
            headers,
        }
    }

    pub fn to_bytes(&mut self) -> Vec<u8> {
        // Update Content-Length header
        self.headers.insert(
            "Content-Length".to_string(),
            vec![self.body.len().to_string()],
        );

        // Construct headers string
        let mut response = Vec::new();

        // Add status line
        response.extend(self.status_line.to_bytes());
        response.extend(b"\r\n");

        // Add headers
        for (name, values) in &self.headers {
            response.extend(format!("{}: ", name).as_bytes());
            if values.len() > 1 {
                response.extend(values.join(", ").as_bytes());
            } else {
                response.extend(values[0].as_bytes());
            }
            response.extend(b"\r\n");
        }

        // Add blank line between headers and body
        response.extend(b"\r\n");

        // Add body
        response.extend(&self.body);

        response
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

use std::{collections::HashMap, net::TcpStream};

use anyhow::bail;

#[derive(Debug, Clone)]
pub struct RequestLine {
    pub method: String,
    pub target: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub request_line: RequestLine,
    pub headers: HashMap<String, Vec<String>>,
    pub body: Vec<u8>,
}
impl Request {
    pub fn new(stream: &TcpStream) -> anyhow::Result<Self> {
        use std::io::{BufRead, BufReader, Read};
        let mut buf_reader = BufReader::new(stream);
        //read start-line into struct
        let mut start_line = String::new();
        buf_reader.read_line(&mut start_line)?;
        let start_line = start_line.trim();
        let parts: Vec<&str> = start_line.split_whitespace().collect();
        if parts.len() != 3 {
            bail!("Invalid request line");
        }
        let request_line = RequestLine {
            method: parts[0].to_string(),
            target: parts[1].to_string(),
            version: parts[2].to_string(),
        };

        let mut request = Request {
            request_line,
            headers: HashMap::new(),
            body: Vec::new(),
        };
        // read each header field line into hash table by field name until empty line
        let mut line = String::new();
        loop {
            line.clear();
            buf_reader.read_line(&mut line)?;
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            let header: Vec<&str> = line.split(":").map(|part| part.trim()).collect();
            request
                .headers
                .entry(header[0].to_string())
                // .and_modify(|values| values.push(header[1].to_string()))
                .or_insert(
                    header[1]
                        .split(",")
                        .map(|value| value.trim().to_string())
                        .collect(),
                );
        }
        // use parsed data to determine if body is expected
        if let Some(content_length) = request.headers.get("Content-Length") {
            // if message expected
            println!("content-length: {:#?}", content_length);
            if let Ok(content_length) = content_length[0].parse::<usize>() {
                // read body until amounts of octets equal to content-length header or connection is
                // closed
                let mut buffer: Vec<u8> = vec![0; content_length];
                buf_reader.read_exact(&mut buffer)?;
                request.body = buffer;
            }
        }

        // println!("request: {:#?}", String::from_utf8(request.body.clone())?);
        Ok(request)
    }
}

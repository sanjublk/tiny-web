mod lib;
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use serde_json::{json, Value};
use tinyweb::Request;
use url::Url;

use lib::Response;

fn main() {
    let addr = "127.0.0.1:34254";
    let listner = TcpListener::bind(addr).unwrap();

    for stream in listner.incoming() {
        let mut stream = stream.unwrap();
        let res = handle_connection(&mut stream);
        stream.write_all(b"meh");
    }
}

pub fn handle_connection(stream: &mut TcpStream) -> Response{
    let mut buffer = [0; 1024];
    let byte_n = stream.read(&mut buffer).unwrap();
    let request = parse(&buffer[0..byte_n]);
    dbg!(request);
    Response {
        body: "Meh, This is my response".to_owned(),
        content_type: "type".to_owned(),
        status_code: 200
    }
}

pub fn parse(buf: &[u8]) -> Request {
    let request = String::from_utf8_lossy(buf).into_owned();
    let request_lines: Vec<&str> = request.split("\r\n").collect();
    let request_line: Vec<&str> = request_lines[0].split(" ").collect();
    let method = request_line[0];
    dbg!(&request_lines);
    let mut body = String::new();
    let mut headers: HashMap<String, String> = HashMap::new();
    for (i, line) in request_lines[1..].iter().enumerate() {
        if line == &"" {
            let json: Result<Value, serde_json::Error> = serde_json::from_str(request_lines[i + 1]);
            body = match json {
                Ok(v) => v.to_string(),
                Err(_) => request_lines[i + 1..request_lines.len()].join(""),
            };
            break;
        }
        let j: Vec<&str> = line.split(":").collect();
        headers.insert(j[0].to_owned(), j[1].to_owned());
    }
    let parsed_url = Url::parse(&format!("https://dummybase.com{}", request_line[1])).unwrap();
    let queries = parsed_url
        .query_pairs()
        .into_owned()
        .collect::<HashMap<String, String>>();
    let path = parsed_url.path().to_owned();

    Request {
        method: method.to_owned(),
        headers: headers,
        path: path,
        queries: queries,
        body: body,
    }
}

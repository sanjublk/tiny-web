use std::{collections::HashMap, net::TcpListener, io::{Read, Write}};

use serde::Deserialize;
use serde_json::Value;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub method: String,
    pub headers: HashMap<String, String>,
    pub path: String,
    pub queries: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub body: String,
    pub status_code: i32,
    pub content_type: String,
}

pub struct TinyWeb {
    route_table: HashMap<String, fn(Request) -> Response>,
}

impl TinyWeb {

    pub fn new() -> Self {
        Self {
            route_table: HashMap::new()
        }
    }

    pub fn run(&self, port: &str) {
        let addr = format!("127.0.0.1:{}", port);
        let listner = TcpListener::bind(addr).unwrap();
        let mut buf = [0; 1024];
        let mut n;
        let mut req: Request;
        let mut res: Response;
        
        for stream in listner.incoming() {
            let mut res: Response = Response {
                status_code:200,
                body:format!("{{meh: {}}}", "ha"),
                content_type:"application/json".to_owned()};
            let mut stream = stream.unwrap();
            req = {
                n = stream.read(&mut buf).unwrap();
                Self::parse_request(&buf[0..n])
            };

            buf.fill(0);
            res = self.get_response(req);
            stream.write_all(Self::build_response(res).as_bytes()).unwrap();
        }

    }

    pub fn add_route(&mut self, route: &str, f: fn(Request) -> Response) {
        self.route_table.insert(route.to_owned(), f);
    }

    fn get_response(&self, req: Request,) -> Response {
        if self.route_table.contains_key(&req.path) {
            self.route_table.get(&req.path).unwrap()(req)
        }
        
        else {
            Response {
                status_code: 404,
                body: format!("<h1>Not Found 404</h1>"),
                content_type: "text/html".to_owned()
            }
        }
    }

    pub fn build_response(response: Response) -> String {
        format!("HTTP/1.1 {}\r\nContent-Type: {}; charset=utf-8\r\nContent-Length: {}\r\nConnection: Closed\r\n\r\n{}", 
        &response.status_code, &response.content_type, &response.body.len(), &response.body)
    }

    pub fn parse_request(buf: &[u8]) -> Request {
        let request = String::from_utf8_lossy(buf).into_owned();
        let request_lines: Vec<&str> = request.split("\r\n").collect();
        let request_line: Vec<&str> = request_lines[0].split(" ").collect();
        let method = request_line[0];
        let mut body = String::new();
        let mut headers: HashMap<String, String> = HashMap::new();
        for (i, line) in request_lines[1..].iter().enumerate() {
            if line == &"" {
                let json: Result<Value, serde_json::Error> =
                    serde_json::from_str(request_lines[i + 1]);
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
        let path = parsed_url.path().trim_end_matches("/").to_owned();

        Request {
            method: method.to_owned(),
            headers: headers,
            path: path,
            queries: queries,
            body: body,
        }
    }
}



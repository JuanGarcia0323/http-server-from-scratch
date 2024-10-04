static SUCCES_RESPONSE_FILE: &'static str = include_str!("./response.txt");
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{Shutdown, TcpListener, TcpStream},
};

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

#[derive(Debug)]
pub struct Endpoint(String);
impl Endpoint {
    pub fn new(route: &str) -> Self {
        Endpoint(String::from(route))
    }
    pub fn get(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct Connection {
    http_request: Vec<String>,
}
impl Connection {
    pub fn new(stream: &mut TcpStream) -> Self {
        let buf_reader = BufReader::new(stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        Connection { http_request }
    }
    pub fn get_method(&self) -> Method {
        let http_request = &self.http_request;
        let first_line: Vec<&str> = match http_request.get(0) {
            Some(r) => r.split(" ").collect(),
            None => vec![],
        };
        let request_method = match first_line.get(0) {
            Some(r) => *r,
            None => "",
        };

        match request_method {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "PATCH" => Method::PATCH,
            _ => Method::GET,
        }
    }
    pub fn get_endpoint(&self) -> Endpoint {
        let http_request = &self.http_request;
        let first_line: Vec<&str> = match http_request.get(0) {
            Some(r) => r.split(" ").collect(),
            None => vec![],
        };
        let route = match first_line.get(1) {
            Some(r) => Some(*r),
            None => None,
        };

        match route {
            Some(r) => Endpoint::new(r),
            _ => Endpoint::new(""),
        }
    }
    pub fn write_response(stream: &mut TcpStream, message: &String) {
        let response = SUCCES_RESPONSE_FILE
            .replace("{{len}}", &message.len().to_string())
            .replace("{{message}}", &message);

        stream.write(&response.as_bytes()).unwrap();
    }
}

type MethodTable<'a> = HashMap<&'a str, fn() -> String>;
pub struct App<'a> {
    put: MethodTable<'a>,
    post: MethodTable<'a>,
    delete: MethodTable<'a>,
    patch: MethodTable<'a>,
    get: MethodTable<'a>,
}
impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            put: HashMap::new(),
            post: HashMap::new(),
            delete: HashMap::new(),
            patch: HashMap::new(),
            get: HashMap::new(),
        }
    }
    pub fn create(&mut self, name: &'a str, method: Method, action: fn() -> String) {
        match method {
            Method::GET => self.get.insert(name, action),
            Method::PUT => self.put.insert(name, action),
            Method::POST => self.post.insert(name, action),
            Method::PATCH => self.patch.insert(name, action),
            Method::DELETE => self.delete.insert(name, action),
        };
    }
    pub fn handle_connection(
        &self,
        endpoint: Endpoint,
        method: &'a Method,
        connection: &mut TcpStream,
    ) {
        let action = *match method {
            Method::GET => self.get.get(endpoint.get()).unwrap(),
            Method::POST => self.post.get(endpoint.get()).unwrap(),
            Method::PUT => self.put.get(endpoint.get()).unwrap(),
            Method::PATCH => self.patch.get(endpoint.get()).unwrap(),
            Method::DELETE => self.delete.get(endpoint.get()).unwrap(),
        };
        let result = action();
        Connection::write_response(connection, &result)
    }
    fn redirect_stream(&self, mut stream: TcpStream) {
        let connection = Connection::new(&mut stream);
        let method = connection.get_method();
        let endpoint = connection.get_endpoint();

        self.handle_connection(endpoint, &method, &mut stream);
    }
    pub fn listen(&self, port: u32) {
        let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.redirect_stream(stream),
                Err(_) => {
                    println!("Not working");
                }
            }
        }
    }
}

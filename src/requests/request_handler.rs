use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{Shutdown, TcpStream},
    rc::Rc,
};

#[derive(Debug)]
pub enum Methods {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

static SUCCES_RESPONSE_FILE: &'static str = include_str!("./response.txt");

#[derive(Debug)]
pub struct Connections {
    stream: TcpStream,
    request: Vec<String>,
    method: Option<Methods>,
    route: Option<String>,
}
impl Connections {
    pub fn new(mut stream: TcpStream) -> Self {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let first_line: Vec<&str> = match http_request.get(0) {
            Some(line) => line.split(" ").collect(),
            None => vec![],
        };

        let method = match first_line.get(0) {
            Some(m) => *m,
            None => "",
        };

        let route = match first_line.get(1) {
            Some(m) => Some(String::from(*m)),
            None => None,
        };

        match stream.write(b"HTTP/1.1 200 OK") {
            Ok(_) => {
                println!("Response was writen")
            }
            _ => stream
                .shutdown(Shutdown::Both)
                .expect("Somenthing went wrong"),
        };

        Connections {
            stream,
            method: match method {
                "GET" => Some(Methods::GET),
                "POST" => Some(Methods::POST),
                "PUT" => Some(Methods::PUT),
                "DELETE" => Some(Methods::DELETE),
                "PATCH" => Some(Methods::PATCH),
                _ => None,
            },
            request: http_request,
            route,
        }
    }

    pub fn write_response(&mut self, message: &str) {
        let response = SUCCES_RESPONSE_FILE
            .replace("{{len}}", &message.len().to_string())
            .replace("{{message}}", &message);

        self.stream.write(&response.as_bytes()).unwrap();
    }

    pub fn get_request(&self) -> String {
        self.request.join("\n")
    }

    pub fn get_route(&self) -> &str {
        match &self.route {
            Some(r) => &r,
            None => "",
        }
    }

    pub fn get_method(&self) -> Methods {
        Methods::GET
    }
}

type MethodTable<'a> = HashMap<&'a str, &'a dyn Fn(&mut TcpStream)>;
pub struct Router<'a> {
    put: MethodTable<'a>,
    post: MethodTable<'a>,
    delete: MethodTable<'a>,
    patch: MethodTable<'a>,
    get: MethodTable<'a>,
}
impl<'a> Router<'a> {
    pub fn new() -> Router<'a> {
        Router {
            put: HashMap::new(),
            post: HashMap::new(),
            delete: HashMap::new(),
            patch: HashMap::new(),
            get: HashMap::new(),
        }
    }
    pub fn create(&mut self, name: &'a str, method: Methods, action: &'a dyn Fn(&mut TcpStream)) {
        match method {
            Methods::GET => self.get.insert(name, action),
            Methods::PUT => self.put.insert(name, action),
            Methods::POST => self.post.insert(name, action),
            Methods::PATCH => self.patch.insert(name, action),
            Methods::DELETE => self.delete.insert(name, action),
        };
    }
    pub fn handle_connection(
        &self,
        name: &'a str,
        method: &'a Methods,
        connection: &mut TcpStream,
    ) {
        let action = *match method {
            Methods::GET => self.get.get(&name).unwrap(),
            Methods::POST => self.post.get(&name).unwrap(),
            Methods::PUT => self.put.get(&name).unwrap(),
            Methods::PATCH => self.patch.get(&name).unwrap(),
            Methods::DELETE => self.delete.get(&name).unwrap(),
        };
        action(connection);
    }
}

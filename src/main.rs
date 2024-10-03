static SUCCES_RESPONSE_FILE: &'static str = include_str!("./requests/response.txt");
pub mod requests;
use requests::request_handler::{Connections, Methods, Router};
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

struct Endpoint {
    route: String,
}
impl Endpoint {
    fn new(route: &str) -> Self {
        Endpoint {
            route: String::from(route),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut router = Router::new();
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let first_line: Vec<&str> = match http_request.get(0) {
        Some(r) => r.split(" ").collect(),
        None => vec![],
    };

    let route = match first_line.get(1) {
        Some(r) => *r,
        None => "",
    };
    let request_method = match first_line.get(0) {
        Some(r) => *r,
        None => "",
    };
    let method = match request_method {
        "GET" => Methods::GET,
        "POST" => Methods::POST,
        "PUT" => Methods::PUT,
        "DELETE" => Methods::DELETE,
        "PATCH" => Methods::PATCH,
        _ => Methods::GET,
    };
    let endpoint = Endpoint::new(route);

    fn test(connection: &mut TcpStream) {
        let message = "We did it";
        let response = SUCCES_RESPONSE_FILE
            .replace("{{len}}", &message.len().to_string())
            .replace("{{message}}", &message);

        connection.write(&response.as_bytes()).unwrap();
    }

    router.create("/otra-ruta/", Methods::GET, &test);
    println!("{}", http_request.join("\n"));
    router.handle_connection(&endpoint.route, &method, &mut stream);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(_) => {
                println!("No anda");
            }
        }
    }
}

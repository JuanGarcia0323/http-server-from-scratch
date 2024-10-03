static SUCCES_RESPONSE_FILE: &'static str = include_str!("./requests/response.txt");
pub mod requests;
use requests::request_handler::{Connection, Method, Router};
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
    let mut connection = Connection::new(&mut stream);
    let method = connection.get_method();
    let endpoint = connection.get_endpoint();

    // let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<String> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // let first_line: Vec<&str> = match http_request.get(0) {
    //     Some(r) => r.split(" ").collect(),
    //     None => vec![],
    // };

    // let route = match first_line.get(1) {
    //     Some(r) => *r,
    //     None => "",
    // };
    // let request_method = match first_line.get(0) {
    //     Some(r) => *r,
    //     None => "",
    // };
    // let method = match request_method {
    //     "GET" => Method::GET,
    //     "POST" => Method::POST,
    //     "PUT" => Method::PUT,
    //     "DELETE" => Method::DELETE,
    //     "PATCH" => Method::PATCH,
    //     _ => Method::GET,
    // };
    // let endpoint = Endpoint::new(route);

    fn test(connection: &mut TcpStream) {
        Connection::write_response(connection, "Test");
    }

    router.create("/otra-ruta/", Method::GET, test);
    router.handle_connection(endpoint, &method, &mut stream);
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

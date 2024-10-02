pub mod requests;
use requests::request_handler::{Connections, Methods, Router};
use std::net::{TcpListener, TcpStream};

fn handle_connection(stream: TcpStream) {
    let mut router = Router::new();
    let connection = Connections::new(stream);

    let method = connection.get_method();
    let route = connection.get_route();

    fn test_get() {
        println!("Test");
    }
    router.create("/otra-ruta/", Methods::GET, test_get);
    router.handle_connection(route, method);
    // println!("Este es el request:\n {}", connection.get_request());
    // let method = connection.get_method().unwrap();
    // let route = connection.get_route().unwrap();

    // match route {
    //     Some(r) => println!("{r}"),
    //     _ => (),
    // }

    // match method {
    //     Some(Methods::GET) => {
    //         println!("Vamos chabon que era un GET!")
    //     }
    //     _ => (),
    // }

    // let test_router = || {
    //     println!("Testing");
    //     connection.write_response("Esto viene del response");
    // };

    // router.create(route, &method, &test_router);
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

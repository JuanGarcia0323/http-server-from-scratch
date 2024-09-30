use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{Shutdown, TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let response =
        fs::read("C:\\Users\\juane\\Desktop\\Rust\\basic-http-server\\http-server\\response.txt")
            .unwrap();
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let request = http_request.join("\n");
    stream.write(&response).unwrap();
    println!("Request: {request}");

    stream.shutdown(Shutdown::Both).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(_) => {
                println!("No anda");
            }
        }
    }
    // match listener.accept() {
    //     Ok((mut socket, _)) => handle_client(&mut socket),
    // let mut buff = String::new();
    // match socket.read_to_string(&mut buff) {
    //     Ok(_) => {
    //         println!("{buff}");
    //     }
    //     Err(_) => socket
    //         .shutdown(Shutdown::Both)
    //         .expect("Something went wrong"),
    // };
    // if buff.contains("GET") {
    //     println!("Es un GET")
    // }
    // Err(_) => (),
    // }

    // for stream in listener.incoming() { handle_client(stream.unwrap());
    // }
}

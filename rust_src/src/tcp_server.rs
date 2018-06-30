use std::net::{TcpListener};
use std::io::{Write};

pub fn server(my_json :&str) {

    //let my_json = format!("{}", "\"id\" : 3");

    println!("here is json");
    println!("{}", my_json);
    println!("that was json");


    let listener = TcpListener::bind("127.0.0.1:8088").unwrap();
    println!("starting a json server on localhost 8088. Use http://localhost:8088/");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let response = my_json.as_bytes();
                stream.write(response).expect("Response failed");
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

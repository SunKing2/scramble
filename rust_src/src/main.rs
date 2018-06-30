extern crate rusqlite;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use rusqlite::Connection;

use std::net::{TcpListener};
use std::io::{Write};

#[derive(Debug)]
#[derive(Serialize)]
struct Person {
    id: i32,
    name: String,
    started: String,
}


fn main() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute("CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  started    TEXT NOT NULL,
                  data            BLOB
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        started: format!("{}", 2018),
    };
    conn.execute("INSERT INTO person (name, started)
                  VALUES (?1, ?2 )",
                 &[&me.name, &me.started]).unwrap();

    let mut stmt = conn.prepare("SELECT id, name, started, data FROM person").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        Person {
            id: row.get(0),
            name: row.get(1),
            started: row.get(2),
        }
    }).unwrap();

    let mut my_json = String::new();
    for person in person_iter {
        my_json =  format!("{:?}", json!(person.unwrap()));
    }

    println!("{}", my_json);


    let listener = TcpListener::bind("127.0.0.1:8088").unwrap();
    println!("starting a json server on localhost 8088. Use http://localhost:8088/");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                //let response = b"Hello World";
                let response = my_json.as_bytes();
                stream.write(response).expect("Response failed");
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
}
}

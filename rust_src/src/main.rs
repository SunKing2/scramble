extern crate rusqlite;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
mod tcp_server;

use rusqlite::Connection;

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
        let p  = json!(person.unwrap());
        my_json = p.to_string();
    }

    println!("here is json");
    println!("{}", my_json);
    println!("that was json");

    tcp_server::server(&my_json);

}

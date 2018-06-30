extern crate rusqlite;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
mod tcp_server;
mod data_store;

use rusqlite::Connection;



fn main() {
    data_store::get_json();
    println!("main...that was json, now passing...");

    //tcp_server::server(&my_json);

}

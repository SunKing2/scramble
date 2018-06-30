extern crate rusqlite;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod tcp_server;
mod data_store;


fn main() {
    data_store::get_json();
    println!("main...that was json, now passing...");

    let my_json = format!("{}", "\"id\" : 10");
    tcp_server::server(&my_json);

}

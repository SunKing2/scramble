extern crate rusqlite;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod tcp_server;
mod data_store;


fn main() {
    let my_json = data_store::get_json();
    println!("main...that was json, now passing...");

    tcp_server::server(&my_json);

}

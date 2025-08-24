use http::Method;
use http::Request;
use server::Server;

mod http; // http folder -> create mod.rs
mod server; // server.rs

fn main() {
    let server = Server::new("127.0.0.1".to_string());
    server.run();
}

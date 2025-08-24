fn main() {
    let server = Server::new("127.0.0.1".to_string());
    server.run();
}

struct Server {
    address: String,
}
impl Server {
    fn new(address: String) -> Self {
        Self { address }
    }

    fn run(self) {
        println!("Server is listening on {}", self.address)
    }
}

use icfpc::mine::Client;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let mut client = Client::new();
    client.execute();
}

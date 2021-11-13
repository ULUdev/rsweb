use rsweb::ressource::RessourceLoader;
use rsweb::server::Server;

fn main() {
    let mut server = Server::new(
        10,
        RessourceLoader::new(10, ".".to_string()),
        8080,
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
    );
    server.run("log.txt");
}

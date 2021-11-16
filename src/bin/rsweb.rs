use rsweb::ressource::RessourceLoader;
use rsweb::route::Router;
use rsweb::server::Server;

fn main() {
    let mut router = Router::new(String::from("/test.html"));
    router.route(String::from("/test"), String::from("/test.html"));
    let mut server = Server::new(
        10,
        RessourceLoader::new(10, ".".to_string()),
        router,
        8080,
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
    );
    server.run("log.txt");
}

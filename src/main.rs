#[macro_use]
extern crate nickel;

use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/test", middleware! { |request, response|
        format!("Hello from GET /test")
    });

    server.utilize(router);

    server.listen("127.0.0.1:9000");
}

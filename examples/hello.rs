use craft;
use hyper::*;

fn hello(_request: &Request<Body>) -> Response<Body> {
    Response::new(Body::from("Hello, World!"))
}

#[runtime::main]
async fn main() {
    let config = craft::Config::new("127.0.0.1", 3000);
    let mut handler_stack = craft::Stack::empty();
    handler_stack.set(hello);
    craft::get("/hello", handler_stack);
    craft::start(&config).await;
}

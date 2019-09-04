use hyper::{Body, Request, Response, StatusCode};
use patricia_router::Router;
use std::sync::RwLock;

use crate::stack::Stack;

lazy_static! {
    static ref ROUTER: RwLock<Router::<Stack>> = RwLock::new(Router::<Stack>::new());
}

pub(crate) fn add(path: &str, stack: Stack) -> () {
    ROUTER.write().unwrap().add(path, stack);
}

pub(crate) fn find(request: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = handler_path(&request);
    if let Some(stack) = ROUTER.read().unwrap().find(path).payload {
        Ok(stack.run(&request))
    } else {
        let mut not_found = Response::default();
        *not_found.status_mut() = StatusCode::NOT_FOUND;
        Ok(not_found)
    }
}

fn handler_path(request: &Request<Body>) -> String {
    let method_name = format!("{:?}", request.method()).to_lowercase();
    "/".to_string() + &method_name + request.uri().path()
}

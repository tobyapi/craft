use hyper::{Body, Request, Response};

pub struct Stack {
    handlers: Vec<Box<fn(&Request<Body>, &dyn Fn() -> Response<Body>) -> Response<Body>>>,
    server_handler: fn(&Request<Body>) -> Response<Body>,
}

impl Stack {
    pub fn new(
        handlers: Vec<Box<fn(&Request<Body>, &dyn Fn() -> Response<Body>) -> Response<Body>>>,
        server_handler: fn(&Request<Body>) -> Response<Body>,
    ) -> Self {
        Self {
            handlers: handlers,
            server_handler: server_handler,
        }
    }

    pub fn empty() -> Self {
        Stack::new(vec![], |_| Response::new(Body::empty()))
    }

    pub fn push(
        &mut self,
        handler: fn(&Request<Body>, &dyn Fn() -> Response<Body>) -> Response<Body>,
    ) -> () {
        self.handlers.push(Box::new(handler));
    }

    pub fn set(&mut self, server_handler: fn(&Request<Body>) -> Response<Body>) -> () {
        self.server_handler = server_handler;
    }

    pub fn run(&self, request: &Request<Body>) -> Response<Body> {
        self.next(0, request)
    }

    fn next(&self, index: usize, request: &Request<Body>) -> Response<Body> {
        if index < self.handlers.len() {
            let next = || -> Response<Body> { self.next(index + 1, request) };
            self.handlers[index](request, &next)
        } else {
            (self.server_handler)(request)
        }
    }
}

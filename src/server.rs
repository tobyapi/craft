use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

use futures::compat::*;
use futures::prelude::*;
use runtime::net::TcpListener;

use crate::config::Config;
use crate::router;

pub async fn run(config: &Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let new_svc = make_service_fn(|_| {
        async { Ok::<_, hyper::Error>(service_fn(router::find)) }
            .boxed()
            .compat()
    });

    let mut listener = TcpListener::bind(&config.addr).unwrap();
    let incoming = listener.incoming().map_ok(Compat::new).compat();
    Server::builder(incoming)
        .executor(Compat::new(runtime::task::Spawner::new()))
        .serve(new_svc)
        .compat()
        .await?;
    Ok(())
}

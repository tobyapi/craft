use crate::config::Config;
use crate::router;
use crate::server;
use crate::stack::Stack;

pub async fn start(config: &Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    server::run(config).await?;
    Ok(())
}

pub fn get(path: &str, stack: Stack) {
    router::add(&("/get".to_string() + path), stack);
    router::add(&("/head".to_string() + path), Stack::empty());
}

pub fn post(path: &str, stack: Stack) {
    router::add(&("/post".to_string() + path), stack);
}

pub fn put(path: &str, stack: Stack) {
    router::add(&("/put".to_string() + path), stack);
}

pub fn delete(path: &str, stack: Stack) {
    router::add(&("/delete".to_string() + path), stack);
}

pub fn patch(path: &str, stack: Stack) {
    router::add(&("/patch".to_string() + path), stack);
}

pub fn options(path: &str, stack: Stack) {
    router::add(&("/options".to_string() + path), stack);
}

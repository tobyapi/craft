pub struct Config {
    pub host: String,
    pub port: i32,
    pub addr: String,
    pub env: String,
    pub logging: bool,
}

impl Config {
    pub fn new(host: &str, port: i32) -> Self {
        let addr = format!("{}:{}", host, port);
        Self {
            host: host.to_string(),
            port: port,
            addr: addr,
            env: "development".to_string(),
            logging: true,
        }
    }
}

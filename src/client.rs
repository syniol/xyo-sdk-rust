use crate::config::ClientConfig;

pub struct Client {
    pub config: ClientConfig,
    http_client: i32,
}

pub fn new_client(config: ClientConfig) -> Client {
    Client{
        config,
        http_client: 0,
    }
}

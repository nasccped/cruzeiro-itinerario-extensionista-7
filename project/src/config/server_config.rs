use crate::helpers;
use std::env;

/// Nome da variável para a url do server.
const SERVER_URL: &str = "SERVER_URL";

/// Nome da variável para a porta do server.
const SERVER_PORT: &str = "SERVER_PORT";

/// Configurações para o [`actix_web::HttpServer`].
pub struct ServerConfig {
    url: String,
    port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let url = env::var(SERVER_URL).unwrap_or_else(|_| helpers::undefined_var_panic(SERVER_URL));
        let port =
            env::var(SERVER_PORT).unwrap_or_else(|_| helpers::undefined_var_panic(SERVER_PORT));
        let port = port
            .parse()
            .unwrap_or_else(|_| helpers::invalid_var_format_panic(SERVER_PORT, port));
        Self { url, port }
    }
}

impl ServerConfig {
    pub fn server_url(&self) -> &str {
        &self.url
    }

    pub fn server_port(&self) -> u16 {
        self.port
    }
}

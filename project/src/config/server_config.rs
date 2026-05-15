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
        let unwrap_or_panic =
            |var: &str| env::var(var).unwrap_or_else(|_| helpers::undefined_var_panic(var));
        let parse = |x: &str| {
            x.parse()
                .unwrap_or_else(|_| helpers::invalid_var_format_panic(SERVER_PORT, x))
        };
        let url = unwrap_or_panic(SERVER_URL);
        let port = unwrap_or_panic(SERVER_PORT);
        let port = parse(port.as_str());
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

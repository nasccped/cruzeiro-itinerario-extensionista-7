mod connection_config;
mod server_config;

use crate::{helpers, usecases::UserUsecases};
use actix_web::web;
use connection_config::ConnectionConfig;
use server_config::ServerConfig;

/// Configurações para toda a aplicação.
pub struct Config {
    server_config: ServerConfig,
    /// Casos de uso para as operações com os usuários.
    pub user_usecases: web::Data<UserUsecases>,
}

impl Config {
    pub async fn new() -> Self {
        let server_config = ServerConfig::default();
        let conn = ConnectionConfig::default()
            .into_pool()
            .await
            .unwrap_or_else(|err| helpers::could_not_build_server(err));
        let user_usecases = web::Data::new(UserUsecases::new(conn));
        Self {
            server_config,
            user_usecases,
        }
    }
}

impl Config {
    /// Retorna a url da aplicação.
    pub fn get_server_url(&self) -> &str {
        self.server_config.server_url()
    }

    /// Retorna a porta da aplicação.
    pub fn get_server_port(&self) -> u16 {
        self.server_config.server_port()
    }
}

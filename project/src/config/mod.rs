mod connection_config;
mod server_config;

use crate::{helpers, usecases::users::UserUsecase};
use actix_web::web;
use connection_config::ConnectionConfig;
use server_config::ServerConfig;

/// Configurações para toda a aplicação.
pub struct Config {
    server_config: ServerConfig,
    /// Casos de uso para as operações com os usuários.
    pub user_usecase: web::Data<UserUsecase>,
}

impl Config {
    pub async fn new() -> Self {
        let server_config = ServerConfig::default();
        let conn = ConnectionConfig::default()
            .into_pool()
            .await
            .unwrap_or_else(|err| helpers::could_not_connect_to_db_panic(err));
        let user_usecase = web::Data::new(UserUsecase::new(conn));
        Self {
            server_config,
            user_usecase,
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

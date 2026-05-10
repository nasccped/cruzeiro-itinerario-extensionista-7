mod server_config;

use crate::usecases::UserUsecases;
use actix_web::web;
use server_config::ServerConfig;

/// Configurações para toda a aplicação.
pub struct Config {
    server_config: ServerConfig,
    /// Casos de uso para as operações com os usuários.
    pub user_usecases: web::Data<UserUsecases>,
}

impl Default for Config {
    fn default() -> Self {
        let server_config = ServerConfig::default();
        let user_usecases = web::Data::new(UserUsecases::default());
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

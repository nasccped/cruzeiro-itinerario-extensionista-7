use crate::helpers;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

const POSTGRES_USER: &str = "POSTGRES_USER";
const POSTGRES_PASSWORD: &str = "POSTGRES_PASSWORD";
const POSTGRES_HOST: &str = "POSTGRES_HOST";
const POSTGRES_PORT: &str = "POSTGRES_PORT";
const POSTGRES_DB_NAME: &str = "POSTGRES_DB_NAME";

/// Configurações para conexão com o Postgres Pool.
pub struct ConnectionConfig {
    username: String,
    password: String,
    db_host: String,
    db_port: u16,
    db_name: String,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        let get_or_panic = |name: &str| {
            env::var(name).unwrap_or_else(|_| {
                eprint!("`ConnectionConfig` error: ");
                helpers::undefined_var_panic(name);
            })
        };
        let username = get_or_panic(POSTGRES_USER);
        let password = get_or_panic(POSTGRES_PASSWORD);
        let db_host = get_or_panic(POSTGRES_HOST);
        let db_port = get_or_panic(POSTGRES_PORT);
        let db_name = get_or_panic(POSTGRES_DB_NAME);
        let db_port = if let Ok(val) = db_port.parse() {
            val
        } else {
            eprint!("`ConnectionConfig` error: ");
            helpers::invalid_var_format_panic(POSTGRES_PORT, db_port);
        };
        Self {
            username,
            password,
            db_host,
            db_port,
            db_name,
        }
    }
}

impl ConnectionConfig {
    /// Retorna a [`ConnectionConfig`] como uma URL.
    pub fn get_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.db_host, self.db_port, self.db_name
        )
    }

    /// Convert o objeto [`ConnectionConfig`] em um [`Result`] de [`PgPool`] or [`sqlx::Error>`].
    pub async fn into_pool(self) -> Result<PgPool, sqlx::Error> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.db_host, self.db_port, self.db_name
        );
        PgPoolOptions::new().connect(&url).await
    }
}

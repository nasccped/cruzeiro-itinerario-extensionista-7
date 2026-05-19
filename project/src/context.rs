use crate::{
    helpers::{FatalPanic, Helper, HelperPanicable},
    usecases::{moderator::ModeratorUsecase, users::UserUsecase},
};
use actix_web::web;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;

const SERVER_URL: &str = "SERVER_URL";
const SERVER_PORT: &str = "SERVER_PORT";
const POSTGRES_USER: &str = "POSTGRES_USER";
const POSTGRES_PASSWORD: &str = "POSTGRES_PASSWORD";
const POSTGRES_HOST: &str = "POSTGRES_HOST";
const POSTGRES_PORT: &str = "POSTGRES_PORT";
const POSTGRES_DATABASE: &str = "POSTGRES_DATABASE";

/// Configurações para toda a aplicação.
pub struct Context {
    /// URL do servidor.
    pub server_url: String,
    /// Porta do servidor.
    pub server_port: u16,
    /// Casos de uso para as operações com os usuários.
    pub user_usecase: web::Data<UserUsecase>,
    /// Casos de uso para as operações com os moderadores.
    pub moderator_usecase: web::Data<ModeratorUsecase>,
}

impl Context {
    pub async fn new() -> Arc<Self> {
        let server_url = get_env_var(SERVER_URL);
        let server_port = parse_env_var(SERVER_PORT);
        let conn = Connection::new().into_pool().await;
        let user_usecase = web::Data::new(UserUsecase::new(conn.clone()));
        let moderator_usecase = web::Data::new(ModeratorUsecase::new(conn));
        Arc::new(Self {
            server_url,
            server_port,
            user_usecase,
            moderator_usecase,
        })
    }
}

/// Struct para conexão com banco de dados.
struct Connection {
    /// Nome do usuário conectado.
    username: String,
    /// Senha do usuário conectado.
    password: String,
    /// Host.
    host: String,
    /// Porta.
    port: u16,
    /// Nome do banco de dados.
    database: String,
}

impl Connection {
    /// Cria uma nova [`Connection`].
    fn new() -> Self {
        let (username, password, host, port, database) = (
            get_env_var(POSTGRES_USER),
            get_env_var(POSTGRES_PASSWORD),
            get_env_var(POSTGRES_HOST),
            parse_env_var(POSTGRES_PORT),
            get_env_var(POSTGRES_DATABASE),
        );
        Self {
            username,
            password,
            host,
            port,
            database,
        }
    }

    /// Converte a [`Connection`] em um [`PgPool`].
    async fn into_pool(self) -> PgPool {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        );
        PgPoolOptions::new()
            .connect(&url)
            .await
            .map_err(HelperPanicable::<()>::CouldNotConnectToDB)
            .ok_or_fatal_panic()
    }
}

/// Retorna a variável de ambiente associada ao nome.
fn get_env_var(var_name: &str) -> String {
    Helper::get_env_var(var_name).ok_or_fatal_panic()
}

/// Retorna a variável de ambiente associada ao nome (já parseada).
fn parse_env_var<T: 'static + std::fmt::Debug + std::str::FromStr>(var_name: &str) -> T {
    Helper::parse_env_var(var_name, Helper::get_env_var(var_name).ok_or_fatal_panic())
        .ok_or_fatal_panic()
}

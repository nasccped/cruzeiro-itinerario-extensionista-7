#![allow(non_snake_case)]
use crate::{
    Context,
    controllers::{HomeController, ModeratorController, UserController},
};
use actix_web::{
    FromRequest, Handler, Responder, Route,
    web::{self, ServiceConfig},
};
use std::{
    any::{Any, type_name},
    env::{self, VarError},
    fmt::Debug,
    marker::PhantomData,
    str::FromStr,
};

/// Alias de função para criar rotas de requisições `get`.
fn Get<F, Args>(handler: F) -> Route
where
    F: Handler<Args>,
    Args: FromRequest + 'static,
    F::Output: Responder + 'static,
{
    web::get().to(handler)
}

/// Alias de função para criar rotas de requisições `post`.
fn Post<F, Args>(handler: F) -> Route
where
    F: Handler<Args>,
    Args: FromRequest + 'static,
    F::Output: Responder + 'static,
{
    web::post().to(handler)
}

/// Alias de função para criar rotas de requisições `delete`.
fn Delete<F, Args>(handler: F) -> Route
where
    F: Handler<Args>,
    Args: FromRequest + 'static,
    F::Output: Responder + 'static,
{
    web::delete().to(handler)
}

/// Alias de função para criar rotas de requisições `patch`.
fn Patch<F, Args>(handler: F) -> Route
where
    F: Handler<Args>,
    Args: FromRequest + 'static,
    F::Output: Responder + 'static,
{
    web::patch().to(handler)
}

pub trait FatalPanic<OkType> {
    /// Retorna o valor interno da variante [`Ok`] (ou executa o [`Err`] apropriado).
    fn ok_or_fatal_panic(self) -> OkType;
}

impl<'a, AnyOk: Any, ParseError: Debug> FatalPanic<AnyOk>
    for Result<AnyOk, HelperPanicable<'a, ParseError>>
{
    fn ok_or_fatal_panic(self) -> AnyOk {
        match self {
            Ok(ok) => ok,
            Err(err) => panic!("{}", err.to_string()),
        }
    }
}

/// Struct que opera as funções auxiliares.
pub struct Helper {}

/// Possíveis erros retornados ao executar funções do [`Helper`].
#[derive(thiserror::Error, Debug)]
pub enum HelperPanicable<'a, VarParseOutput: Debug> {
    /// Quando a função de [`dotenv::dotenv`] falha.
    #[error("Falha ao executar função `dotenv`: {:?}", .0)]
    DotEnv(dotenv::Error),
    /// Variável de ambiente não existe.
    #[error("Variável de ambiente `{}` é indefinida!", .0)]
    UndefinedEnvVar(&'a str),
    /// Variável de ambiente não é unicode válido.
    #[error("Variável de ambiente `{}` não é unicode válido!", .0)]
    NonUnicodeEnvVar(&'a str),
    /// Quando a variável de ambiente existe mas não é do tipo esperado.
    #[error(
        "Variável de ambiente `{}` não é um {} válido: {}",
        .var_name,
        type_name::<VarParseOutput>(),
        .var_value
    )]
    CouldNotParseEnvVar {
        /// Nome da variável.
        var_name: &'a str,
        /// Valor da variável.
        var_value: String,
        /// Phatom data para [`type_name`] printing.
        out_type: PhantomData<VarParseOutput>,
    },
    /// Quando a construção do servidor falha.
    #[error("Não foi possível construir o servidor http: {:?}", .0)]
    CouldNotBuildServer(std::io::Error),
    /// Quando há um erro durante a conexão com o banco de dados.
    #[error("Não foi possível conectar com o banco de dados: {:?}", .0)]
    CouldNotConnectToDB(sqlx::Error),
}

impl Helper {
    /// Tenta executar [`dotenv::dotenv`]. Executa [`panic`] caso haja falha.
    pub fn setup_dotenv<'a>() -> Result<(), HelperPanicable<'a, ()>> {
        dotenv::dotenv()
            .map(|_| ())
            .map_err(HelperPanicable::DotEnv)
    }

    /// Inicializa o logger.
    pub fn init_logger() {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("project=info"))
            .init();
    }

    /// Obtem a variável de ambiente como uma [`String`] (ou o [`HelperPanicable`] apropriado).
    pub fn get_env_var<'a>(var_name: &'a str) -> Result<String, HelperPanicable<'a, ()>> {
        env::var(var_name).map_err(|e| match e {
            VarError::NotPresent => HelperPanicable::UndefinedEnvVar(var_name),
            VarError::NotUnicode(_) => HelperPanicable::NonUnicodeEnvVar(var_name),
        })
    }

    /// Faz _parsing_ de uma [`String`] para outro tipo. Caso falhe, retorna o [`HelperPanicable`]
    /// apropriado.
    pub fn parse_env_var<'a, Output: Debug + FromStr>(
        var_name: &'a str,
        var_value: String,
    ) -> Result<Output, HelperPanicable<'a, Output>> {
        var_value
            .parse::<Output>()
            .map_err(|_| HelperPanicable::CouldNotParseEnvVar {
                var_name,
                var_value,
                out_type: PhantomData,
            })
    }

    /// Configura o [`App`] por meio de um [`ServiceConfig`].
    pub fn configure(service: &mut ServiceConfig, gctx: &Context) {
        Self::add_app_data(service, gctx);
        Self::add_home_routes(service);
        Self::add_users_routes(service);
        Self::add_moderators_routes(service);
    }

    /// Adiciona os `AppData` à configuração.
    fn add_app_data(service: &mut ServiceConfig, gctx: &Context) {
        service
            .app_data(gctx.user_usecase.clone())
            .app_data(gctx.moderator_usecase.clone());
    }

    /// Adiciona as rotas de [`HomeController`].
    fn add_home_routes(service: &mut ServiceConfig) {
        service.route(
            HomeController::app_home_endpoint(),
            Get(HomeController::app_home),
        );
    }

    /// Adiciona as rotas de [`UserController`].
    fn add_users_routes(service: &mut ServiceConfig) {
        service
            .route(
                UserController::get_users_endpoint(),
                Get(UserController::get_users),
            )
            .route(
                UserController::get_user_by_id_endpoint(),
                Get(UserController::get_user_by_id),
            )
            .route(
                UserController::post_user_endpoint(),
                Post(UserController::post_user),
            )
            .route(
                UserController::patch_user_endpoint(),
                Patch(UserController::patch_user),
            );
    }

    /// Adiciona as rotas de [`ModeratorController`].
    fn add_moderators_routes(service: &mut ServiceConfig) {
        service
            .route(
                ModeratorController::get_moderators_endpoint(),
                Get(ModeratorController::get_moderators),
            )
            .route(
                ModeratorController::post_moderator_endpoint(),
                Post(ModeratorController::post_moderator),
            )
            .route(
                ModeratorController::delete_moderator_endpoint(),
                Delete(ModeratorController::delete_moderator),
            );
    }
}

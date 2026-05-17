use crate::{
    config::Config,
    controllers::{HomeController, ModeratorController, UserController},
};
use actix_web::{
    App, Error,
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    web,
};
use std::fmt::{Debug, Display};

/// Tenta executar [`dotenv::dotenv`]. Executa [`panic`] caso haja falha.
pub fn setup_dotenv() {
    dotenv::dotenv().unwrap_or_else(|e| panic!("Falha ao executar função `dotenv`: {e:?}"));
}

/// Inicializa o logger.
pub fn init_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("project=info"))
        .init();
}

/// Função de [`panic`] para quando variável de ambiente não é definida.
pub fn undefined_var_panic(name: impl Display) -> ! {
    panic!("Variável de ambiente ${name} não definida");
}

/// Função de [`panic`] para quando variável de ambiente possui valor inválido.
pub fn invalid_var_format_panic(name: impl Display, value: impl Display) -> ! {
    panic!("Variável de ambiente ${name} possui valor inválido: {value}");
}

/// Quando a construção do [`actix_web::HttpServer`] retorna [`Err`].
pub fn could_not_build_server(err: impl Debug) -> ! {
    panic!("Não foi possível gerar o servidor: {err:?}");
}

/// Mensagem de [`panic`] quando não é possível conectar com o banco de dados.
pub fn could_not_connect_to_db_panic<D: Debug>(err: D) -> ! {
    panic!("Não foi possível connectar com o banco de dados => {err:?}");
}

/// Cria um novo [`App`] padrão para o projeto.
pub fn create_app(
    config: web::Data<Config>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let mut app = App::new()
        .app_data(config.user_usecase.clone())
        .app_data(config.moderator_usecase.clone());
    let endpoints_and_routes = [
        (
            HomeController::get_app_home_endpoint(),
            web::get().to(HomeController::app_home),
        ),
        (
            UserController::get_users_endpoint(),
            web::get().to(UserController::get_users),
        ),
        (
            UserController::get_user_by_id_endpoint(),
            web::get().to(UserController::get_user_by_id),
        ),
        (
            UserController::post_user_endpoint(),
            web::post().to(UserController::post_user),
        ),
        (
            ModeratorController::get_moderators_endpoint(),
            web::get().to(ModeratorController::get_moderators),
        ),
        (
            ModeratorController::post_moderator_endpoint(),
            web::post().to(ModeratorController::post_moderator),
        ),
    ];
    for (endpoint, route) in endpoints_and_routes {
        app = app.route(endpoint, route);
    }
    app
}

use crate::{
    config::Config,
    controllers::{self, UserControllers},
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
    let user_usecases = config.user_usecases.clone();
    let mut app = App::new().app_data(user_usecases).app_data(config.clone());
    let endpoints_and_routes = [
        (
            controllers::get_app_home_endpoint(),
            web::get().to(controllers::app_home),
        ),
        (
            UserControllers::get_users_endpoint(),
            web::get().to(UserControllers::get_users),
        ),
        (
            UserControllers::get_user_by_id_endpoint(),
            web::get().to(UserControllers::get_user_by_id),
        ),
    ];
    for (endpoint, route) in endpoints_and_routes {
        app = app.route(endpoint, route);
    }
    app
}

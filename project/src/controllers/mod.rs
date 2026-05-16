mod user_controllers;
mod utils;

use actix_web::{HttpResponse, Responder};
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};
pub use user_controllers::UserControllers;

/// Retorna o endpoint para [`app_home`].
pub fn get_app_home_endpoint() -> &'static str {
    "/"
}

/// Rota para o endpoint vazio (`/`).
pub async fn app_home() -> impl Responder {
    let mut response = "URL inicial. Considere utilizar os demais endpoints:\n\n".to_string();
    let endpoints: HashSet<&str> = [
        UserControllers::get_users_endpoint(),
        UserControllers::get_user_by_id_endpoint(),
    ]
    .into();
    response.push_str(
        &endpoints
            .iter()
            .map(|edp| format!("- {}", edp))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    HttpResponse::Ok().body(response)
}

/// Função base para logs de info.
fn log_info(endpoint: impl Display, response: impl Debug) {
    log::info!(
        "requisição no endpoint `{}` retorna `{:?}`",
        endpoint,
        response
    );
}

/// Função base para logs de erro.
fn log_error(endpoint: impl Display, response: impl Debug) {
    log::error!(
        "requisição no endpoint `{}` retorna `{:?}`",
        endpoint,
        response
    );
}

trait LogAndSelf<T> {
    /// Loga as informações e retorna si mesmo ao final do log.
    fn log_and_self(self, endpoint: impl Display) -> T;
}

impl LogAndSelf<HttpResponse> for Result<HttpResponse, HttpResponse> {
    fn log_and_self(self, endpoint: impl Display) -> HttpResponse {
        self.inspect(|resp| log_info(&endpoint, resp))
            .inspect_err(|resp| log_error(endpoint, resp))
            .unwrap_or_else(|err| err)
    }
}

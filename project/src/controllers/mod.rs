mod user_controllers;

use actix_web::{HttpResponse, Responder};
use std::collections::HashSet;
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

trait LogAndSelf {
    /// Loga as informações e retorna si mesmo ao final do log.
    fn log_and_self(self, endpoint: String, err: bool) -> Self;
}

impl LogAndSelf for HttpResponse {
    fn log_and_self(self, endpoint: String, err: bool) -> Self {
        let string = format!("acesso no endpoint `{}` retorna {:?}", endpoint, self);
        if err {
            log::error!("{}", string);
        } else {
            log::info!("{}", string);
        }
        self
    }
}

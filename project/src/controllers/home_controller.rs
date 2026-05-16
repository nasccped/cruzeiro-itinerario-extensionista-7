use super::UserControllers;
use super::utils;
use actix_web::HttpResponse;
use std::collections::HashSet;

/// Struct para o endpoint raíz (`/`).
pub struct Home {}

type HomeResult = Result<HttpResponse, HttpResponse>;

impl Home {
    /// Retorna o endpoint para [`Home::app_home`].
    pub fn get_app_home_endpoint() -> &'static str {
        "/"
    }

    /// Rota para o endpoint vazio (`/`).
    pub async fn app_home() -> HttpResponse {
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
        utils::log_and_normalize(
            HomeResult::Ok(HttpResponse::Ok().body(response)),
            Self::get_app_home_endpoint(),
        )
    }
}

use super::log_helper::LogAndSelf;
use crate::{controllers::utils::IntoHttpResponseResult, usecases::UserUsecases};
use actix_web::{HttpResponse, web};

const USERS_ENDPOINT: &str = "/users";
const USERS_AND_USER_ID_ENDPOINT: &str = "/users/{userId}";

pub struct UserControllers {}

impl UserControllers {
    /// Retorna o endpoint para [`UserControllers::get_users`].
    pub fn get_users_endpoint() -> &'static str {
        USERS_ENDPOINT
    }

    /// Retorna o endpoint para [`UserControllers::get_user_by_id`].
    pub fn get_user_by_id_endpoint() -> &'static str {
        USERS_AND_USER_ID_ENDPOINT
    }

    /// Retorna o endpoint para [`UserControllers::post_user`].
    pub fn post_user_endpoint() -> &'static str {
        USERS_ENDPOINT
    }

    /// Retorna uma lista com todos os usuários.
    pub async fn get_users(usecase: web::Data<UserUsecases>) -> HttpResponse {
        usecase
            .get_users()
            .await
            .into_http_response()
            .log_and_self(Self::get_users_endpoint())
    }

    /// Retorna o usuário especificado pelo id.
    pub async fn get_user_by_id(
        usecase: web::Data<UserUsecases>,
        user_id: web::Path<String>,
    ) -> HttpResponse {
        let id = user_id.as_str();
        let endpoint = Self::get_user_by_id_endpoint().replace("{userId}", id);
        usecase
            .get_user_by_id(id)
            .await
            .into_http_response()
            .log_and_self(endpoint)
    }

    /// Adiciona um novo usuário ao banco de dados.
    pub async fn post_user(_usecase: web::Data<UserUsecases>) -> HttpResponse {
        HttpResponse::Ok().body(format!(
            "TODO: endpoint `{}` acessado. Criar novo usuário!",
            Self::post_user_endpoint()
        ))
    }
}

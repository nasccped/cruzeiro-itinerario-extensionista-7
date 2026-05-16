use super::utils;
use crate::usecases::UserUsecases;
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
        utils::log_and_normalize(usecase.get_users().await, Self::get_users_endpoint())
    }

    /// Retorna o usuário especificado pelo id.
    pub async fn get_user_by_id(
        usecase: web::Data<UserUsecases>,
        user_id: web::Path<String>,
    ) -> HttpResponse {
        utils::log_and_normalize(
            usecase.get_user_by_id(user_id.as_str()).await,
            Self::get_user_by_id_endpoint().replace("{userId}", user_id.as_str()),
        )
    }

    /// Adiciona um novo usuário ao banco de dados.
    pub async fn post_user(usecase: web::Data<UserUsecases>, body: String) -> HttpResponse {
        utils::log_and_normalize(usecase.post_user(body).await, Self::post_user_endpoint())
    }
}

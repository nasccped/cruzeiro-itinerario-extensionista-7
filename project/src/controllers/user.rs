use super::_utils as utils;
use crate::usecases::users::UserUsecase;
use actix_web::{HttpResponse, web};

const USERS_ENDPOINT: &str = "/users";
const USERS_AND_USER_ID_ENDPOINT: &str = "/users/{userId}";

pub struct UserController {}

impl UserController {
    /// Retorna o endpoint para [`UserController::get_users`].
    pub fn get_users_endpoint() -> &'static str {
        USERS_ENDPOINT
    }

    /// Retorna o endpoint para [`UserController::get_user_by_id`].
    pub fn get_user_by_id_endpoint() -> &'static str {
        USERS_AND_USER_ID_ENDPOINT
    }

    /// Retorna o endpoint para [`UserController::post_user`].
    pub fn post_user_endpoint() -> &'static str {
        USERS_ENDPOINT
    }

    /// Retorna o endpoint para [`UserController::patch_user`].
    pub fn patch_user_endpoint() -> &'static str {
        USERS_AND_USER_ID_ENDPOINT
    }

    /// Retorna uma lista com todos os usuários.
    pub async fn get_users(usecase: web::Data<UserUsecase>) -> HttpResponse {
        utils::log_and_normalize(usecase.get_users().await, Self::get_users_endpoint())
    }

    /// Retorna o usuário especificado pelo id.
    pub async fn get_user_by_id(
        usecase: web::Data<UserUsecase>,
        user_id: web::Path<String>,
    ) -> HttpResponse {
        utils::log_and_normalize(
            usecase.get_user_by_id(user_id.as_str()).await,
            Self::get_user_by_id_endpoint().replace("{userId}", user_id.as_str()),
        )
    }

    /// Adiciona um novo usuário ao banco de dados.
    pub async fn post_user(usecase: web::Data<UserUsecase>, body: String) -> HttpResponse {
        utils::log_and_normalize(usecase.post_user(body).await, Self::post_user_endpoint())
    }

    /// Atualiza os dados de um usuário.
    pub async fn patch_user(usecase: web::Data<UserUsecase>, body: String) -> HttpResponse {
        utils::log_and_normalize(usecase.patch_user(body).await, Self::patch_user_endpoint())
    }
}

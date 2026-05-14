use crate::{controllers::LogAndSelf, usecases::UserUsecases};
use actix_web::{HttpResponse, Responder, web};

pub struct UserControllers {}

impl UserControllers {
    /// Retorna o endpoint para [`UserControllers::get_users`].
    pub fn get_users_endpoint() -> &'static str {
        "/users"
    }

    /// Retorna o endpoint para [`UserControllers::get_user_by_id`].
    pub fn get_user_by_id_endpoint() -> &'static str {
        "/users/{userId}"
    }

    /// Retorna uma lista com todos os usuários.
    pub async fn get_users(usecase: web::Data<UserUsecases>) -> HttpResponse {
        let endpoint = Self::get_users_endpoint().into();
        match usecase.get_users().await {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(e) => HttpResponse::InternalServerError().body(e),
        }
        .log_and_self(endpoint, false)
    }

    /// Retorna o usuário especificado pelo id.
    pub async fn get_user_by_id(
        usecase: web::Data<UserUsecases>,
        user_id: web::Path<String>,
    ) -> impl Responder {
        let response = usecase.get_user_by_id(user_id).await;
        HttpResponse::Ok().json(response)
    }
}

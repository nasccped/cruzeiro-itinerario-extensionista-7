use crate::usecases::UserUsecases;
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
    pub async fn get_users(usecase: web::Data<UserUsecases>) -> impl Responder {
        let response = usecase.get_users().await;
        HttpResponse::Ok().body(response.to_string())
    }

    /// Retorna o usuário especificado pelo id.
    pub async fn get_user_by_id(
        usecase: web::Data<UserUsecases>,
        user_id: web::Path<String>,
    ) -> impl Responder {
        let response = usecase.get_user_by_id(user_id).await;
        HttpResponse::Ok().body(response)
    }
}

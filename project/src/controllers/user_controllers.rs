use actix_web::{HttpResponse, Responder, web::Path};

/// Controller para as operações com os usuários.
pub struct UserController {}

impl UserController {
    /// Retorna uma lista com todos os usuários.
    pub async fn get_users() -> impl Responder {
        HttpResponse::Ok().body("Vamos retornar alguns usuários")
    }

    /// Retorna o usuário especificado pelo id.
    pub async fn get_user_by_id(user_id: Path<String>) -> impl Responder {
        HttpResponse::Ok().body(format!("Retornando usuário de id {}", user_id))
    }
}

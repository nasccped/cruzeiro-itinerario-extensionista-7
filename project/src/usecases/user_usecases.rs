use actix_web::web;

/// Casos de uso para as operações com os usuários.
#[derive(Default)]
pub struct UserUsecases {}

impl UserUsecases {
    /// Retorna uma lista com todos os usuários.
    pub async fn get_users(&self) -> &str {
        "Lista de usuários"
    }

    /// Retorna o usuário especificado pelo id.
    pub async fn get_user_by_id(&self, user_id: web::Path<String>) -> String {
        format!("Retornando usuário de id {}", user_id)
    }
}

use crate::{
    models::{
        ModelParseError,
        users::{User, Users},
    },
    repositories::UserRepository,
};
use actix_web::web;
use sqlx::{FromRow, PgPool};

/// Casos de uso para as operações com os usuários.
pub struct UserUsecases {
    repository: UserRepository,
}

impl UserUsecases {
    /// Cria uma nova instância de [`UserUsecases`] a partir de uma conexão postgres já
    /// estabelecida.
    pub fn new(conn: PgPool) -> Self {
        let repository = UserRepository::from(conn);
        Self { repository }
    }

    /// Retorna uma lista com todos os usuários.
    pub async fn get_users(&self) -> Result<Users, String> {
        let result = self
            .repository
            .get_users()
            .await
            .map_err(|e| e.to_string())?;
        let mut users = Users::empty();
        for row in result {
            let user = User::from_row(&row).map_err(|e| {
                ModelParseError::from(e)
                    .with_model_name::<User>()
                    .with_input(row)
                    .to_string()
            })?;
            users.push(user);
        }
        Ok(users)
    }

    /// Retorna o usuário especificado pelo id.
    pub async fn get_user_by_id(&self, user_id: web::Path<String>) -> String {
        format!("Retornando usuário de id {}", user_id)
    }
}

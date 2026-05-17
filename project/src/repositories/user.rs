use super::error::RepositoryError;
use crate::models::user::CreateUserModel;
use sqlx::{PgPool, postgres::PgRow};

const SELECT_FROM_USERS_QUERY: &str =
    include_str!("../../db-templates/callables/select-from-users.sql");
const SELECT_FROM_USERS_BY_ID_QUERY: &str =
    include_str!("../../db-templates/callables/select-from-users-by-id.sql");
const INSERT_USER_QUERY: &str = include_str!("../../db-templates/callables/insert-user.sql");

/// Repositório para as operações relacionadas com [`Users`].
pub struct UserRepository {
    /// Conexão com o postgres pool.
    conn: PgPool,
}

impl From<PgPool> for UserRepository {
    fn from(value: PgPool) -> Self {
        Self { conn: value }
    }
}

impl UserRepository {
    /// Retorna os usuários da tabela [`crate::models::users::Users`].
    pub async fn get_users(&self) -> Result<Vec<PgRow>, RepositoryError> {
        sqlx::query(SELECT_FROM_USERS_QUERY)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| RepositoryError::from_err_and_query(e, SELECT_FROM_USERS_BY_ID_QUERY))
    }

    /// Retorna um usuário específico da tabela [`crate::models::users::Users`].
    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<PgRow>, RepositoryError> {
        sqlx::query(SELECT_FROM_USERS_BY_ID_QUERY)
            .bind(id)
            .fetch_optional(&self.conn)
            .await
            .map_err(|e| RepositoryError::from_err_and_query(e, SELECT_FROM_USERS_BY_ID_QUERY))
    }

    /// Adiciona um novo usuário na table [`crate::models::users::Users`].
    pub async fn post_user(&self, model: CreateUserModel) -> Result<(), RepositoryError> {
        sqlx::query(INSERT_USER_QUERY)
            .bind(model.name())
            .bind(model.mail())
            .execute(&self.conn)
            .await
            .map(|_| ())
            .map_err(|e| RepositoryError::from_err_and_query(e, INSERT_USER_QUERY))
    }
}

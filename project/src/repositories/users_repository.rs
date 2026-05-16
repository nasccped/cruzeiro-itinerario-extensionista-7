use super::error::DbOperationError;
use sqlx::{PgPool, postgres::PgRow};

const SELECT_FROM_USERS_QUERY: &str = include_str!("../../db-templates/select-from-users.sql");
const SELECT_FROM_USERS_BY_ID_QUERY: &str =
    include_str!("../../db-templates/select-from-users-by-id.sql");

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
    pub async fn get_users(&self) -> Result<Vec<PgRow>, DbOperationError> {
        sqlx::query(SELECT_FROM_USERS_QUERY)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| DbOperationError::from_err_and_query(e, SELECT_FROM_USERS_BY_ID_QUERY))
    }

    /// Retorna um usuário específico daa tabela [`crate::models::users::Users`].
    pub async fn get_user_by_id(&self, id: i64) -> Result<Option<PgRow>, DbOperationError> {
        sqlx::query(SELECT_FROM_USERS_BY_ID_QUERY)
            .bind(id)
            .fetch_optional(&self.conn)
            .await
            .map_err(|e| DbOperationError::from_err_and_query(e, SELECT_FROM_USERS_BY_ID_QUERY))
    }
}

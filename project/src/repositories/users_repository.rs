use super::DbOperationError;
use sqlx::{PgPool, postgres::PgRow};

const SELECT_FROM_USERS_QUERY: &str = include_str!("../../db-templates/select-from-users.sql");

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
            .map_err(|e| DbOperationError::from(e).with_query(SELECT_FROM_USERS_QUERY))
    }
}

use crate::models::users::{User, Users};
use sqlx::{FromRow, PgPool};

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
    /// Retorna os usuários da tabela [`Users`].
    pub async fn get_users(&self) -> Result<Users, sqlx::Error> {
        let query_result = sqlx::query(SELECT_FROM_USERS_QUERY)
            .fetch_all(&self.conn)
            .await?;
        let mut users = Users::empty();
        query_result.iter().try_for_each(|row| {
            let user = User::from_row(row)?;
            users.push(user);
            Ok::<(), sqlx::Error>(())
        })?;
        Ok(users)
    }
}

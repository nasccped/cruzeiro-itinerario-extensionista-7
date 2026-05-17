use super::error::RepositoryError;
use sqlx::{PgPool, postgres::PgRow};

const SELECT_FROM_MODERATORS_VIEW_QUERY: &str =
    include_str!("../../db-templates/select-from-moderators-view.sql");

/// Repositório para as operações com moderadores.
pub struct ModeratorRepository {
    conn: PgPool,
}

impl ModeratorRepository {
    /// Cria um novo repositório a partir de uma [`PgPool`].
    pub fn new(conn: PgPool) -> Self {
        Self { conn }
    }

    /// Retorna as linhas de um `ModeratorView`.
    pub async fn get_moderators_view(&self) -> Result<Vec<PgRow>, RepositoryError> {
        sqlx::query(SELECT_FROM_MODERATORS_VIEW_QUERY)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| RepositoryError::from_err_and_query(e, SELECT_FROM_MODERATORS_VIEW_QUERY))
    }
}

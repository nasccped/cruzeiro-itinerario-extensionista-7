use super::error::RepositoryError;
use sqlx::{PgPool, postgres::PgRow};

const SELECT_FROM_MODERATORS_VIEW_QUERY: &str =
    include_str!("../../db-templates/callables/select-from-moderators-view.sql");
const INSERT_INTO_MODERATORS_QUERY: &str =
    include_str!("../../db-templates/callables/insert-moderator.sql");

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

    /// Tenta inserir um usuário na tabela de moderadores, retornando uma [`PgRow`] armazenando um
    /// [`crate::models::moderator::ModeratorInsertionReturnType`] (ou um [`RepositoryError`] caso
    /// haja falha do DB).
    pub async fn post_moderator(&self, id: i32) -> Result<PgRow, RepositoryError> {
        sqlx::query(INSERT_INTO_MODERATORS_QUERY)
            .bind(id)
            .fetch_one(&self.conn)
            .await
            .map_err(|e| RepositoryError::from_err_and_query(e, INSERT_INTO_MODERATORS_QUERY))
    }
}

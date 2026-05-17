#![allow(clippy::from_over_into)]
use actix_web::HttpResponse;

/// Quando ocorre um erro (no banco de dados) por meio dos repositories (no banco de dados).
#[derive(thiserror::Error, Debug)]
#[error(
    "Erro ao executar query '{}' no banco de dados. Retornou:\n\
    \n\
    {}",
    .on_query,
    .err
)]
pub struct RepositoryError {
    err: sqlx::Error,
    on_query: String,
}

impl RepositoryError {
    /// Retorna um [`DbOperationError`] a partir de um dador erro + query.
    pub fn from_err_and_query(err: sqlx::Error, query: impl ToString) -> Self {
        let on_query = query.to_string();
        Self { err, on_query }
    }

    /// Retorna o erro interno do [`DbOperationError`].
    pub fn error(&self) -> &sqlx::Error {
        &self.err
    }
}

impl Into<HttpResponse> for RepositoryError {
    fn into(self) -> HttpResponse {
        HttpResponse::InternalServerError().body(self.to_string())
    }
}

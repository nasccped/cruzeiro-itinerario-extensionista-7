#![allow(clippy::from_over_into)]
use actix_web::HttpResponse;

/// Quando ocorre um erro na operação (no banco de dados).
pub struct DbOperationError {
    error: sqlx::Error,
    query: String,
}

impl DbOperationError {
    /// Retorna um [`DbOperationError`] a partir de um dador erro + query.
    pub fn from_err_and_query(error: sqlx::Error, query: impl ToString) -> Self {
        Self {
            error,
            query: query.to_string(),
        }
    }
}

impl Into<HttpResponse> for DbOperationError {
    fn into(self) -> HttpResponse {
        HttpResponse::InternalServerError().body(self.to_string())
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for DbOperationError {
    fn to_string(&self) -> String {
        format!(
            "Erro ao executar query '{}' no banco.\n\
            \n\
            Erro retornado: {}",
            self.query, self.error
        )
    }
}

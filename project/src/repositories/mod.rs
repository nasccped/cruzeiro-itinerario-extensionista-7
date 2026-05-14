mod users_repository;

pub use users_repository::UserRepository;

/// Quando ocorre um erro na operação (no banco de dados).
pub struct DbOperationError {
    query: Option<String>,
    error: sqlx::Error,
}

impl From<sqlx::Error> for DbOperationError {
    fn from(value: sqlx::Error) -> Self {
        Self {
            query: None,
            error: value,
        }
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for DbOperationError {
    fn to_string(&self) -> String {
        format!(
            "Erro ao operar `{}` no banco de dados: {}",
            self.query.as_deref().unwrap_or("<QUERY INDEFINIDA>"),
            self.error
        )
    }
}

impl DbOperationError {
    /// Especifica qual foi a query executada.
    fn with_query<T: ToString>(mut self, query: T) -> Self {
        self.query = Some(query.to_string());
        self
    }
}

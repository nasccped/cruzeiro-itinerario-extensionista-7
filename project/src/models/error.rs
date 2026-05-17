use actix_web::HttpResponse;
use std::{
    any::{Any, type_name},
    fmt::Debug,
    marker::PhantomData,
};

/// Erro quando parse de struct falha.
#[derive(thiserror::Error, Debug)]
#[error(
    "Não foi possível deserializar o objeto `{}` a partir do input `{:?}`.\n\
    \n\
    Tentativa retorna seguinte erro: {}",
    type_name::<Model>(),
    .input,
    .error
)]
pub struct ModelParseError<Model: Any + Debug, Input: Debug> {
    input: Input,
    error: sqlx::Error,
    _marker: PhantomData<Model>,
}

impl<Model: Any + Debug, Input: Debug> ModelParseError<Model, Input> {
    /// Cria um [`ModelParseError`] a partir de um [`sqlx::Error`] e um input.
    pub fn from_err_and_input(error: sqlx::Error, input: Input) -> Self {
        Self {
            input,
            error,
            _marker: PhantomData,
        }
    }
}

impl<Model: Any + Debug, Input: Debug> From<ModelParseError<Model, Input>> for HttpResponse {
    fn from(val: ModelParseError<Model, Input>) -> Self {
        HttpResponse::UnprocessableEntity().body(val.to_string())
    }
}

#![allow(clippy::from_over_into)]
use actix_web::HttpResponse;
use std::{
    any::{Any, type_name},
    fmt::Debug,
    marker::PhantomData,
};

/// Erro quando parse de struct falha.
pub struct ModelParseError<Model: Any, Input: Debug> {
    input: Input,
    error: sqlx::Error,
    _marker: PhantomData<Model>,
}

impl<Model: Any, Input: Debug> ModelParseError<Model, Input> {
    /// Cria um [`ModelParseError`] a partir de um [`sqlx::Error`] e um input.
    pub fn from_err_and_input(error: sqlx::Error, input: Input) -> Self {
        Self {
            input,
            error,
            _marker: PhantomData,
        }
    }
}

impl<Model: Any, Input: Debug> Into<HttpResponse> for ModelParseError<Model, Input> {
    fn into(self) -> HttpResponse {
        HttpResponse::InternalServerError().body(self.to_string())
    }
}

#[allow(clippy::to_string_trait_impl)]
impl<Model: Any, Input: Debug> ToString for ModelParseError<Model, Input> {
    fn to_string(&self) -> String {
        format!(
            "Não foi possível deserializar o objeto `{}` a partir do input `{:?}`.\n\
            \n\
            Tentativa retorna seguinte erro: {}",
            type_name::<Model>(),
            self.input,
            self.error
        )
    }
}

pub mod users;

use sqlx::Error;
use std::{any::Any, fmt::Debug};

/// Erro quando parse de struct falha.
pub struct ModelParseError<D: Debug> {
    model_name: Option<&'static str>,
    input: Option<D>,
    err: Error,
}

impl<D: Debug> From<Error> for ModelParseError<D> {
    fn from(value: Error) -> Self {
        Self {
            model_name: None,
            input: None,
            err: value,
        }
    }
}

impl<D: Debug> ModelParseError<D> {
    /// Especifica o nome do model onde o parser falhou.
    pub fn with_model_name<T: Any>(mut self) -> Self {
        self.model_name = Some(std::any::type_name::<T>());
        self
    }

    /// Especifica o input utilizado (debug mode) durante a falha do parsing.
    pub fn with_input(mut self, input: D) -> Self {
        self.input = Some(input);
        self
    }
}

#[allow(clippy::to_string_trait_impl)]
impl<D: Debug> ToString for ModelParseError<D> {
    fn to_string(&self) -> String {
        format!(
            "Não for possível parsear o objeto `{}` por meio do input => {:?}.\n\
            Tentativa resulta em => {}",
            self.model_name.unwrap_or("<INDEFINIDO>"),
            self.input,
            self.err
        )
    }
}

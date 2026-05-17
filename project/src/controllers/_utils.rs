use actix_web::HttpResponse;
use std::fmt::{Debug, Display};

/// Realiza o logging e a normalização comum encontrada em todos os controllers do projeto.
pub fn log_and_normalize(
    value: impl HttpResponseResultPair,
    endpoint: impl Display,
) -> HttpResponse {
    let result = value.into_pair();
    result.log_response(endpoint);
    result.unwrap_or_else(|err| err)
}

/// Trait comum para conversão de tipos em par de [`Result`] ([`HttpResponse`]).
pub trait HttpResponseResultPair {
    /// Converte objeto para par de [`Result`] ([`HttpResponse`]).
    fn into_pair(self) -> Result<HttpResponse, HttpResponse>;
}

impl<T: Into<HttpResponse>, E: Into<HttpResponse>> HttpResponseResultPair for Result<T, E> {
    fn into_pair(self) -> Result<HttpResponse, HttpResponse> {
        self.map(|ok| ok.into()).map_err(|err| err.into())
    }
}

/// Função base para logs de info.
fn log_info(endpoint: impl Display, response: impl Debug) {
    log::info!(
        "requisição no endpoint `{}` retorna `{:?}`",
        endpoint,
        response
    );
}

/// Função base para logs de erro.
fn log_error(endpoint: impl Display, response: impl Debug) {
    log::error!(
        "requisição no endpoint `{}` retorna `{:?}`",
        endpoint,
        response
    );
}

trait LogResponse {
    /// Loga as informações.
    fn log_response(&self, endpoint: impl Display);
}

impl LogResponse for Result<HttpResponse, HttpResponse> {
    fn log_response(&self, endpoint: impl Display) {
        match self {
            Ok(resp) => log_info(endpoint, resp),
            Err(resp) => log_error(endpoint, resp),
        }
    }
}

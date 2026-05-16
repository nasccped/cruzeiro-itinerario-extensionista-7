use actix_web::HttpResponse;
use std::fmt::{Debug, Display};

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

pub trait LogAndSelf<T> {
    /// Loga as informações e retorna si mesmo ao final do log.
    fn log_and_self(self, endpoint: impl Display) -> T;
}

impl LogAndSelf<HttpResponse> for Result<HttpResponse, HttpResponse> {
    fn log_and_self(self, endpoint: impl Display) -> HttpResponse {
        self.inspect(|resp| log_info(&endpoint, resp))
            .inspect_err(|resp| log_error(endpoint, resp))
            .unwrap_or_else(|err| err)
    }
}

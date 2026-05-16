use actix_web::HttpResponse;

/// Trait comum para conversão de [`HttpResponse`].
pub trait IntoHttpResponseResult {
    /// Converte todo [`Result<T, E>`] (desde que `T` e `E` implementem [`Into<HttpResponse>`]) em
    /// [`Result<HttpResponse, HttpResponse>`].
    fn into_http_response(self) -> Result<HttpResponse, HttpResponse>;
}

impl<T: Into<HttpResponse>, E: Into<HttpResponse>> IntoHttpResponseResult for Result<T, E> {
    fn into_http_response(self) -> Result<HttpResponse, HttpResponse> {
        self.map(|ok| ok.into()).map_err(|err| err.into())
    }
}

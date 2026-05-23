pub type ErrorType = Result<actix_web::HttpResponse, actix_web::HttpResponse>;

/// Macro para retornar um [`actix_web::HttpResponse::NotImplemented`].
#[macro_export]
macro_rules! not_implemented {
    () => {{
        let err: $crate::global_not_implemented::ErrorType =
            Err(actix_web::HttpResponse::NotImplemented()
                .body("Funcionalidade ainda não implementada!"));
        err
    }};
}

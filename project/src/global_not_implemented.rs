/// Macro para retornar um [`actix_web::HttpResponse::NotImplemented`].
#[macro_export]
macro_rules! not_implemented {
    () => {
        Err(actix_web::HttpResponse::NotImplemented()
            .body("Funcionalidade ainda não implementada!"))
    };
}

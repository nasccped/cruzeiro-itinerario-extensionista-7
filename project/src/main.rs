use actix_web::{App, HttpResponse, HttpServer, Responder, get, main as async_main};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Olá!")
}

#[async_main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(home))
        .bind(("127.0.0.1", 8080))
        .unwrap_or_else(|e| panic!("expected success but http server bound returned: {e}"));
    println!("server successfully bound to 127.0.0.1:8080 :^D");
    server.run().await
}

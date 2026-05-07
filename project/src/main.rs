use actix_web::{App, HttpResponse, HttpServer, Responder, get, main as async_main};
use dotenv::dotenv;
use std::env;

const SERVER_URL: &str = "SERVER_URL";
const SERVER_PORT: &str = "SERVER_PORT";

/// Executa [`panic`] quando não é possível ler a variável de ambiente.
fn unreadable_var_panic(var_name: &str) -> ! {
    panic!("Não foi possível ler a varíavel ${}", var_name);
}

/// Retorna a URL do servidor.
fn get_server_url() -> String {
    env::var(SERVER_URL).unwrap_or_else(|_| unreadable_var_panic(SERVER_URL))
}

/// Retorna a porta do servidor.
fn get_server_port() -> u16 {
    let port = env::var(SERVER_PORT).unwrap_or_else(|_| unreadable_var_panic(SERVER_PORT));
    if let Ok(p) = port.parse() {
        p
    } else {
        panic!(
            "o variável ${} não possui um valor apropriado: {}",
            SERVER_PORT, port
        );
    }
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Olá!")
}

#[async_main]
async fn main() {
    dotenv().ok();
    let server_url = get_server_url();
    let server_port = get_server_port();
    let server = HttpServer::new(|| App::new().service(home))
        .bind((server_url.clone(), server_port))
        .unwrap_or_else(|e| panic!("Falha ao gerar o servidor: {:?}", e));
    println!("Servidor rodando em `{}:{}`", server_url, server_port);
    server.run().await.unwrap()
}

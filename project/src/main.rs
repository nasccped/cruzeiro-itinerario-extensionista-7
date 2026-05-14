mod config;
mod controllers;
mod helpers;
mod models;
mod repositories;
mod usecases;

use actix_web::{HttpServer, web};
use config::Config;

#[actix_web::main]
async fn main() {
    helpers::setup_dotenv();
    helpers::init_logger();
    let conf = web::Data::new(Config::new().await);
    let clone = conf.clone();
    let server = HttpServer::new(move || helpers::create_app(clone.clone()))
        .bind((conf.get_server_url(), conf.get_server_port()))
        .unwrap_or_else(|e| helpers::could_not_build_server(e));
    log::info!(
        "Servidor rodando em `{}:{}`",
        conf.get_server_url(),
        conf.get_server_port()
    );
    server.run().await.unwrap()
}

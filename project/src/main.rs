mod context;
mod controllers;
mod helpers;
mod models;
mod repositories;
mod usecases;

use crate::helpers::{FatalPanic, HelperPanicable};
use actix_web::{App, HttpServer};
use context::Context;
use helpers::Helper;

/// Inicialize os recursos essencias da aplicação (logger, dotenv).
fn init_core() {
    Helper::setup_dotenv().ok_or_fatal_panic();
    Helper::init_logger();
}

#[actix_web::main]
async fn main() {
    init_core();
    let gctx = Context::new().await;
    let clone = gctx.clone();
    let (url, port) = (gctx.server_url.as_str(), gctx.server_port);
    let server = HttpServer::new(move || App::new().configure(|sv| Helper::configure(sv, &clone)))
        .bind((url, port))
        .map_err(HelperPanicable::<()>::CouldNotBuildServer)
        .ok_or_fatal_panic();
    log::info!("Servidor rodando em `{}:{}`", url, port);
    server.run().await.unwrap()
}

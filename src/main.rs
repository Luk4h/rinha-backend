use actix_web::{middleware, get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    valor: i32,
    tipo: String,
    descricao: String,
}

#[get("/clientes/{id}/extrato")]
async fn extrato(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[post("/clientes/{id}/transacoes")]
async fn transactions(name: web::Path<String>, item: web::Json<MyObj>) -> impl Responder {
    println!("model: {:?}", &name);
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .app_data(web::JsonConfig::default().limit(4096))
        .service(extrato)
        .service(transactions)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
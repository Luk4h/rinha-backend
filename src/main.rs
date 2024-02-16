use actix_web::{body::BoxBody, get, middleware, post, web::{self, Json}, App, HttpResponse, HttpServer, Responder};
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use serde::{Deserialize, Serialize};

mod actions;

/// Short-hand for the database pool type to use throughout the app.
type DbPool = r2d2::Pool<PostgresConnectionManager<NoTls>>;

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    valor: f32,
    tipo: char,
    descricao: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransactionResponse {
    limite: f32,
    saldo: f32
}

#[derive(Debug, Serialize, Deserialize)]
struct Cliente {
    limite: f32,
    saldo: f32
}

#[post("/clientes/{id}/transacoes")]
async fn transactions(
    id: web::Path<i32>,
    item: web::Json<MyObj>,
    pool: web::Data<DbPool>
) -> impl Responder {

    let item = item.0;
    let value = item.valor;
    let tipo = item.tipo;
    println!("tipo: {}", &tipo);

    let pool_clone = pool.clone();
    let id_clone: i32 = id.clone();
    // Get user
    let user = 
    actions::get_user(&pool_clone, &id_clone).await;
    let user = match user {
        Ok(user) => user,
        Err(_e) => return HttpResponse::BadRequest().body(String::from("usuário não encontrado.")),
    };

    let limite = user.get("limite");
    let saldo_atual: f32 = user.get("saldo");
    println!("Saldo atual: {}", &saldo_atual);
    match tipo {
        'c' => {
            let novo_saldo = saldo_atual + value;
            actions::update_value(&pool, &id, novo_saldo).await;
            return HttpResponse::Ok().json(web::Json(TransactionResponse{ limite, saldo: novo_saldo}))
        },
        'd' => {
            let novo_saldo = saldo_atual - value;
            actions::update_value(&pool, &id, saldo_atual - value).await;
            return HttpResponse::Ok().json(web::Json(TransactionResponse{ limite, saldo: novo_saldo}))
        },
        _ => HttpResponse::BadRequest().body(String::from("método não encontrado."))
    }
}

// #[get("/clientes/{id}/extrato")]
// async fn extrato(
//     id: web::Path<i32>,
//     pool: web::Data<DbPool>,
// ) -> actix_web::Result<impl Responder> {

//     // Get user
//     let user = web::block(move || actions::get_user(&pool, &id))
//     .await?;
//     let user = match user {
//         Ok(user) => user,
//         Err(_e) => return Ok(HttpResponse::BadRequest().message_body(String::from("usuário não encontrado."))),
//     };


//     let user_balance: f32 = user.get("saldo");
//     Ok(HttpResponse::Ok().message_body(format!("Your balance is: {}", &user_balance)))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initialize DB pool outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();

    HttpServer::new(move || {
        App::new()
        .wrap(middleware::Logger::default())
        .app_data(web::JsonConfig::default().limit(4096))
        .app_data(web::Data::new(pool.clone()))
        // .service(extrato)
        .service(transactions)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


fn initialize_db_pool() -> DbPool {
    let manager = PostgresConnectionManager::new(
        "host=dev.lgvm.dev user=postgres password=rinha-backend".parse().unwrap(),
        NoTls,
    );
    let pool = r2d2::Pool::new(manager).unwrap();

    pool
}
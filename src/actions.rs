use actix_web::web;
use r2d2::Pool;
use r2d2_postgres::{postgres::{Error, NoTls, Row}, PostgresConnectionManager};

pub async fn update_value(pool: &Pool<PostgresConnectionManager<NoTls>>, id: &i32, value: f32 ) -> Result<u64, Error> {
  let user_id = id.clone();
  let pool = pool.clone();
  let value = value.clone();
  web::block(move || {
    println!("Usuário {}, novo saldo: {}", &user_id, &value);
    let mut conn = pool.get().unwrap();
    conn.execute("UPDATE clientes SET saldo = $1 WHERE id = $2", &[&value, &user_id])
  }).await.unwrap()
}

pub async fn get_user(pool: &Pool<PostgresConnectionManager<NoTls>>, id: &i32) -> Result<Row, Error>{
  let user_id = id.clone();
  let pool = pool.clone();
  web::block(move || {
    let mut conn = pool.get().unwrap();
    conn.query_one("SELECT * FROM clientes WHERE id = $1", &[&user_id])
  }).await.unwrap()
}
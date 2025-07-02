use actix_web::{App, HttpServer};
use rinha_2024_rs::clients::{
  statements::client_statement, transactions::client_transaction,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(client_transaction)
      .service(client_statement)
  })
  .bind(("127.0.0.1", 5000))
  .inspect(|s| println!("server listening on: {:?}", s.addrs_with_scheme()))?
  .run()
  .await
}

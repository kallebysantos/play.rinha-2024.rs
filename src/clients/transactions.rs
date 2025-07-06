use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};

use crate::{
  clients::{Client, Transaction, TransactionKind},
  db::establish_connection,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(remote = "TransactionKind")]
pub enum TransactionKindPayload {
  #[serde(rename = "c")]
  Credit,
  #[serde(rename = "d")]
  Debit,
}

#[derive(Deserialize, Debug)]
pub struct ClientTransactionPayload {
  #[serde(rename = "valor")]
  value: usize,
  #[serde(rename = "tipo", with = "TransactionKindPayload")]
  kind: TransactionKind,
  #[serde(rename = "descricao")]
  description: String,
}

#[derive(Serialize, Debug)]
pub struct ClientTransactionResult {
  #[serde(rename = "limite")]
  limit: usize,
  #[serde(rename = "saldo")]
  balance: isize,
}

#[post("/clientes/{id}/transacoes")]
async fn client_transaction(
  id: web::Path<usize>,
  data: web::Json<ClientTransactionPayload>,
) -> impl Responder {
  let id = id.into_inner();
  let data = data.into_inner();

  if data.description.is_empty() || data.description.len() > 10 {
    return HttpResponse::BadRequest().body("invalid description");
  }

  let mut conn = establish_connection();
  let Ok(Some(mut client)) = Client::find(&mut conn, id) else {
    return HttpResponse::NotFound().finish();
  };

  let transaction = Transaction::new(data.value, data.kind, data.description);

  if let Err(e) = client.apply_transaction(&mut conn, transaction) {
    eprintln!("Error while processing transaction: {e:?}");
    return HttpResponse::UnprocessableEntity().finish();
  };

  println!("client: {client:#?} was sucessfully updated");

  HttpResponse::Ok().json(ClientTransactionResult {
    limit: client.limit,
    balance: client.balance,
  })
}

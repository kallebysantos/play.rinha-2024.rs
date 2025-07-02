use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};

use crate::clients::{CLIENTS, Transaction, TransactionKind};

#[derive(Deserialize, Debug)]
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

  let mut client = {
    let Some(client_guard) = CLIENTS.get(&id) else {
      return HttpResponse::NotFound().finish();
    };

    client_guard.lock().unwrap()
  };

  let transaction = Transaction::new(data.value, data.kind, data.description);

  if let Err(e) = client.execute_transaction(transaction) {
    eprintln!("Error while processing transaction: {e:?}");
    return HttpResponse::UnprocessableEntity().finish();
  }

  println!("client: {client:#?} was sucessfully updated");

  HttpResponse::Ok().json(ClientTransactionResult {
    limit: client.limit,
    balance: client.balance,
  })
}

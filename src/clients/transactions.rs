use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};

use crate::clients::CLIENTS;

#[derive(Deserialize, Debug)]
enum TransactionKind {
  #[serde(rename = "c")]
  Credit,
  #[serde(rename = "d")]
  Debit,
}

#[derive(Deserialize, Debug)]
pub struct ClientTransactionPayload {
  #[serde(rename = "valor")]
  value: usize,
  #[serde(rename = "tipo")]
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

  if data.description.is_empty() || data.description.len() > 10 {
    return HttpResponse::BadRequest().body("invalid description");
  }

  let mut client = {
    let Some(client_guard) = CLIENTS.get(&id) else {
      return HttpResponse::NotFound().finish();
    };

    client_guard.lock().unwrap()
  };

  match data.kind {
    TransactionKind::Credit => client.credit(data.value),
    TransactionKind::Debit => {
      if let Err(e) = client.debit(data.value) {
        eprintln!("Error while processing transaction: {e:?}");
        return HttpResponse::UnprocessableEntity().finish();
      }
    }
  };

  println!("client: {} was sucessfully updated", client.id);

  HttpResponse::Ok().json(ClientTransactionResult {
    limit: client.limit,
    balance: client.balance,
  })
}

use actix_web::{HttpResponse, Responder, get, web};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::clients::transactions::TransactionKindPayload;
use crate::clients::{Client, TransactionKind};
use crate::db::establish_connection;

#[derive(Serialize, Debug)]
pub struct TransactionResult {
  #[serde(rename = "valor")]
  value: usize,
  #[serde(rename = "tipo", with = "TransactionKindPayload")]
  kind: TransactionKind,
  #[serde(rename = "descricao")]
  description: String,
  #[serde(rename = "realizada_em")]
  timestamp: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct StatementBalanceResult {
  total: isize,
  #[serde(rename = "limite")]
  limit: usize,
  #[serde(rename = "data_extrato")]
  timestamp: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct StatementResult {
  #[serde(rename = "saldo")]
  balance: StatementBalanceResult,
  #[serde(rename = "ultimas_transacoes")]
  last_transactions: Vec<TransactionResult>,
}

#[get("/clientes/{id}/extrato")]
async fn client_statement(id: web::Path<usize>) -> impl Responder {
  let id = id.into_inner();

  let mut conn = establish_connection();
  let Ok(Some(client)) = Client::find_with_transactions(&mut conn, id) else {
    return HttpResponse::NotFound().finish();
  };

  let last_transactions = client
    .transactions
    .iter()
    .rev()
    .map(|i| TransactionResult {
      value: i.value,
      kind: i.kind.clone(),
      timestamp: i.timestamp.unwrap(),
      description: i.description.clone(),
    })
    .take(5)
    .collect();

  let result = StatementResult {
    balance: StatementBalanceResult {
      total: client.balance,
      limit: client.limit,
      timestamp: Utc::now(),
    },
    last_transactions,
  };

  HttpResponse::Ok().json(result)
}

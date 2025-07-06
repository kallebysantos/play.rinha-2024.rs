use chrono::{DateTime, Utc};
use std::str::FromStr;

use crate::{
  clients::{Client, Transaction, TransactionKind},
  schema::{clients, transactions},
};
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = clients)]
pub struct DbClient {
  pub id: i32,
  pub limit: i32,
  pub balance: i32,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(DbClient, foreign_key = client_id))]
#[diesel(table_name = transactions)]
pub struct DbTransaction {
  pub id: i32,
  pub client_id: i32,
  pub value: i32,
  pub kind: String,
  pub description: String,
  pub timestamp: Option<DateTime<Utc>>,
}

impl From<DbClient> for Client {
  fn from(db_value: DbClient) -> Self {
    Self {
      id: db_value.id as usize,
      limit: db_value.limit as usize,
      balance: db_value.balance as isize,
      transactions: vec![], // Filled separately after query
    }
  }
}

impl From<DbTransaction> for Transaction {
  fn from(db: DbTransaction) -> Self {
    let kind =
      TransactionKind::from_str(&db.kind).unwrap_or(TransactionKind::Debit);
    Self {
      value: db.value as usize,
      kind,
      description: db.description,
      timestamp: db.timestamp,
    }
  }
}

impl FromStr for TransactionKind {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_ascii_lowercase().as_str() {
      "c" | "credit" => Ok(TransactionKind::Credit),
      "d" | "debit" => Ok(TransactionKind::Debit),
      _ => Err(()),
    }
  }
}

impl Client {
  pub fn find(
    conn: &mut PgConnection,
    client_id: usize,
  ) -> QueryResult<Option<Client>> {
    let client = Client::load(conn, client_id).optional()?.map(Client::from);

    Ok(client)
  }

  pub fn find_with_transactions(
    conn: &mut PgConnection,
    client_id: usize,
  ) -> QueryResult<Option<Client>> {
    let client = Client::load(conn, client_id)?;

    let transactions = DbTransaction::belonging_to(&client)
      .select(DbTransaction::as_select())
      .load::<DbTransaction>(conn)?;

    let mut client = Client::from(client);

    client.transactions =
      transactions.into_iter().map(Transaction::from).collect();

    Ok(Some(client))
  }

  fn load(conn: &mut PgConnection, client_id: usize) -> QueryResult<DbClient> {
    clients::table
      .filter(clients::id.eq(client_id as i32))
      .select(DbClient::as_select())
      .get_result(conn)
  }
}

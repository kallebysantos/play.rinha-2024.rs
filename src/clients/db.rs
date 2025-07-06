use chrono::{DateTime, Utc};
use std::str::FromStr;

use crate::{
  clients::{Client, ClientError, Transaction, TransactionKind},
  schema::{clients, transactions},
};
use diesel::{insert_into, prelude::*, update};

#[derive(Debug)]
pub enum DbClientError {
  Base(ClientError),
  DbError(diesel::result::Error),
  MissingTransactionTimestamp,
}

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = clients)]
pub struct DbClient {
  pub id: i32,
  pub limit: i32,
  pub balance: i32,
}

#[derive(
  Debug, Queryable, Selectable, Insertable, Identifiable, Associations,
)]
#[diesel(belongs_to(DbClient, foreign_key = client_id))]
#[diesel(table_name = transactions)]
pub struct DbTransaction {
  pub id: i32,
  pub client_id: i32,
  pub value: i32,
  pub kind: String,
  pub description: String,
  pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = transactions)]
pub struct NewDbTransaction {
  pub client_id: i32,
  pub value: i32,
  pub kind: String,
  pub description: String,
  pub timestamp: DateTime<Utc>,
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
      timestamp: Some(db.timestamp),
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

  pub fn apply_transaction(
    &mut self,
    conn: &mut PgConnection,
    transaction: Transaction,
  ) -> Result<(), DbClientError> {
    let client_id = self.id as i32;
    let transaction = self
      .execute_transaction(transaction)
      .map_err(DbClientError::Base)?;

    let transaction = NewDbTransaction {
      client_id,
      description: transaction.description.clone(),
      kind: format!("{:?}", transaction.kind),
      value: transaction.value as i32,
      timestamp: transaction
        .timestamp
        .ok_or(DbClientError::MissingTransactionTimestamp)?,
    };

    update(clients::table.filter(clients::id.eq(client_id)))
      .set(clients::balance.eq(self.balance as i32))
      .execute(conn)
      .map_err(DbClientError::DbError)?;

    insert_into(transactions::table)
      .values(&transaction)
      .returning(DbTransaction::as_returning())
      .get_result(conn)
      .map_err(DbClientError::DbError)?;

    Ok(())
  }
}

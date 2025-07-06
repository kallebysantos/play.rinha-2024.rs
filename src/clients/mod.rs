use std::str::FromStr;

use chrono::{DateTime, Utc};

mod db;
pub mod statements;
pub mod transactions;

#[derive(Debug, PartialEq)]
pub enum ClientError {
  OverLimit,
  TransactionUnref,
}

#[derive(Debug, Clone)]
pub enum TransactionKind {
  Credit,
  Debit,
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

#[derive(Debug)]
pub struct Transaction {
  value: usize,
  kind: TransactionKind,
  description: String,
  timestamp: Option<DateTime<Utc>>,
}

impl Transaction {
  pub fn new(value: usize, kind: TransactionKind, description: String) -> Self {
    Self {
      value,
      kind,
      description,
      timestamp: None,
    }
  }
}

#[derive(Debug)]
pub struct Client {
  id: usize,
  limit: usize,
  balance: isize,
  transactions: Vec<Transaction>,
}

fn is_valid_balance(balance: isize, limit: usize) -> Result<(), ClientError> {
  if balance < -(limit as isize) {
    return Err(ClientError::OverLimit);
  }

  Ok(())
}

impl Client {
  pub fn new(
    id: usize,
    limit: usize,
    balance: isize,
  ) -> Result<Self, ClientError> {
    is_valid_balance(balance, limit)?;

    Ok(Self {
      id,
      limit,
      balance,
      transactions: Vec::new(),
    })
  }

  pub fn execute_transaction(
    &mut self,
    mut transaction: Transaction,
  ) -> Result<&Transaction, ClientError> {
    match transaction.kind {
      TransactionKind::Credit => self.credit(transaction.value),
      TransactionKind::Debit => self.debit(transaction.value)?,
    }

    transaction.timestamp = Some(Utc::now());

    self.transactions.push(transaction);
    let transaction_ref = self
      .transactions
      .last()
      .ok_or(ClientError::TransactionUnref)?;

    Ok(transaction_ref)
  }

  fn debit(&mut self, value: usize) -> Result<(), ClientError> {
    let new_balance = self.balance - (value as isize);

    is_valid_balance(new_balance, self.limit)?;

    self.balance = new_balance;

    Ok(())
  }

  fn credit(&mut self, value: usize) {
    self.balance += value as isize;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn valid_balance() {
    assert_eq!(is_valid_balance(0, 1000), Ok(()));
    assert_eq!(is_valid_balance(1000, 1000), Ok(()));
    assert_eq!(is_valid_balance(-1000, 1000), Ok(()));

    assert_eq!(is_valid_balance(-1001, 1000), Err(ClientError::OverLimit));
  }

  #[test]
  fn client_debit() {
    let mut client = Client::new(1, 1000, 0).unwrap();

    assert_eq!(client.debit(1000), Ok(()));
    assert_eq!(client.debit(1), Err(ClientError::OverLimit));
    assert_eq!(client.balance, -1000);
  }

  #[test]
  fn client_credit() {
    let mut client = Client::new(1, 1000, 0).unwrap();

    client.credit(15);
    client.credit(10);
    assert_eq!(client.balance, 15 + 10);
  }
}

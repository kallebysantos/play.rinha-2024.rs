use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use once_cell::sync::Lazy;

pub mod transactions;

// mock in-memory client list
static CLIENTS: Lazy<HashMap<usize, Arc<Mutex<Client>>>> = Lazy::new(|| {
  let mut map = HashMap::new();
  map.insert(1, Arc::new(Mutex::new(Client::new(1, 1000, 0).unwrap())));
  map.insert(2, Arc::new(Mutex::new(Client::new(2, 1000, 500).unwrap())));
  map.insert(3, Arc::new(Mutex::new(Client::new(3, 1000, -900).unwrap())));
  map
});

#[derive(Debug, PartialEq)]
pub enum ClientError {
  OverLimit,
}

pub struct Client {
  id: usize,
  limit: usize,
  balance: isize,
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

    Ok(Self { id, limit, balance })
  }

  pub fn debit(&mut self, value: usize) -> Result<(), ClientError> {
    let new_balance = self.balance - (value as isize);

    is_valid_balance(new_balance, self.limit)?;

    self.balance = new_balance;

    Ok(())
  }

  pub fn credit(&mut self, value: usize) {
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

pub mod transactions;

#[derive(Debug, PartialEq)]
enum ClientError {
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
    let mut client = Client {
      id: 1,
      limit: 1000,
      balance: 0,
    };

    assert_eq!(client.debit(1000), Ok(()));
    assert_eq!(client.debit(1), Err(ClientError::OverLimit));
    assert_eq!(client.balance, -1000);
  }

  #[test]
  fn client_credit() {
    let mut client = Client {
      id: 1,
      limit: 1000,
      balance: 0,
    };

    client.credit(15);
    client.credit(10);
    assert_eq!(client.balance, 15 + 10);
  }
}

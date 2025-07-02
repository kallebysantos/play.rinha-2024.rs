pub mod transactions;

pub struct Client {
  id: usize,
  limit: usize,
  balance: isize,
}

impl Client {
  pub fn debit(&mut self, value: usize) -> Result<(), &str> {
    let new_balance = self.balance - (value as isize);

    if new_balance < -(self.limit as isize) {
      return Err("value over limit");
    }

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
  fn client_debit() {
    let mut client = Client {
      id: 1,
      limit: 1000,
      balance: 0,
    };

    assert_eq!(client.debit(1000), Ok(()));
    assert_eq!(client.debit(1), Err("value over limit"));
    assert_eq!(client.balance, -1000);
  }
}

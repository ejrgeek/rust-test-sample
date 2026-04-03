use std::panic;

pub struct Store {
    pub balance: f64,
}

impl Store {
    pub fn new() -> Self {
        Self { balance: 0.0 }
    }

    pub fn sell(&mut self, sale_price: f64) -> Result<f64, String> {
        if sale_price < 0.0 {
            return Err(String::from("The sale price must be greater than zero."));
        }

        self.balance += sale_price;

        Ok(sale_price)
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            panic!("Insufficient funds: You tried to withdraw more than your balance allows!");
        }
        self.balance -= amount;
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

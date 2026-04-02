use std::panic;

pub struct Store {
    balance: f64,
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn store_created_with_zero_balance() {
        let expected_balance = 0.0;
        let store: Store = Store::new();
        assert_eq!(expected_balance, store.balance());
    }

    #[test]
    fn sale_increases_balance() {
        let expected_balance = 100.0;
        let mut store: Store = Store::new();
        store.sell(expected_balance).unwrap();

        assert_eq!(expected_balance, store.balance());
    }

    #[test]
    fn multiple_sales_accumulate_balance() {
        let expected_balance = 300.0;
        let mut store: Store = Store::new();
        store.sell(100.0).unwrap();
        store.sell(100.0).unwrap();
        store.sell(100.0).unwrap();

        assert_eq!(expected_balance, store.balance());
    }

    #[test]
    fn must_fail_sale_negative_value() {
        let mut store: Store = Store::new();
        let result = store.sell(-10.0);

        assert!(result.is_err());
    }

    #[test]
    fn balance_not_modified_with_negative_sale_value() {
        let expected_balance = 0.0;
        let mut store: Store = Store::new();
        let _ = store.sell(-40.0);

        assert_eq!(expected_balance, store.balance());
    }

    #[test]
    fn to_withdraw() {
        let expected_balance = 0.0;
        let mut store: Store = Store::new();
        store.sell(100.0).unwrap();
        store.withdraw(100.0);
        assert_eq!(expected_balance, store.balance());
    }

    #[test]
    #[should_panic(
        expected = "Insufficient funds: You tried to withdraw more than your balance allows!"
    )]
    fn withdrawal_of_an_amount_greater_than_the_balance() {
        let mut store: Store = Store::new();
        store.sell(100.0).unwrap();
        store.withdraw(110.0);
    }
}

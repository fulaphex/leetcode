struct Bank {
    balance: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if account1.max(account2) > self.balance.len() as i32 {
            return false;
        }
        if self.balance[(account1 - 1) as usize] < money {
            return false;
        }
        self.balance[(account1 - 1) as usize] -= money;
        self.balance[(account2 - 1) as usize] += money;
        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if account > self.balance.len() as i32 {
            return false;
        }
        self.balance[(account - 1) as usize] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if account > self.balance.len() as i32 {
            return false;
        }
        if self.balance[(account - 1) as usize] < money {
            return false;
        }
        self.balance[(account - 1) as usize] -= money;
        true
    }
}

/**
 * Your Bank object will be instantiated and called as such:
 * let obj = Bank::new(balance);
 * let ret_1: bool = obj.transfer(account1, account2, money);
 * let ret_2: bool = obj.deposit(account, money);
 * let ret_3: bool = obj.withdraw(account, money);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let init_balance = vec![10, 100, 20, 50, 30];
        let mut bank = Bank::new(init_balance);
        assert!(bank.withdraw(3, 10));
        assert!(bank.transfer(5, 1, 20));
        assert!(bank.deposit(5, 20));
        assert!(!bank.transfer(3, 4, 15));
        assert!(!bank.withdraw(10, 50));
    }
}

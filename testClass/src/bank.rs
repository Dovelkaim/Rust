struct Bank{
    name:String,
    account:Vec<Account>,
    transaction:Vec<Transaction>,
}
impl Bank{
    pub fn new(name:String)->Self{
        Self{
            name,
            account:Vec::new(),
            transaction:Vec::new(),
        }
    }

    pub fn add_account(&mut self, account:Account){
        self.account.push(account);
    }
    pub fn print(&self){
        println!("Bank name: {}", self.name);
        println!("Accounts:");
        for account in &self.account{
            account.print();
        }
        println!("Transactions:");
        for transaction in &self.transaction{
            transaction.print();
        }
    }
    pub fn find_account_index(&self, number: u32) -> u32 {
        for (index, account) in self.account.iter().enumerate() {
            if account.number == number {
                return index as u32;
            }
        }
        u32
    }
    pub fn transfer(&mut self, from: u32, to: u32, amount: i32) -> bool {
        let mut tx = Transaction::new(from, to, amount);

        let from_idx = self.find_account_index(from);
        let to_idx = self.find_account_index(to);

        if from_idx == -1 || to_idx == -1 {
            self.transactions.push(tx);
            return false;
        }

        let fi = from_idx as usize;
        let ti = to_idx as usize;

        if fi == ti {
            if self.accounts[fi].balance >= amount {
                tx.approve();
                self.transactions.push(tx);
                return true;
            } else {
                self.transactions.push(tx);
                return false;
            }
        }

        if fi < ti {
            let (left, right) = self.accounts.split_at_mut(ti);
            let from_acc = &mut left[fi];
            let to_acc = &mut right[0];

            if from_acc.balance >= amount {
                from_acc.withdraw(amount);
                to_acc.deposit(amount);
                tx.approve();
                self.transactions.push(tx);
                return true;
            } else {
                self.transactions.push(tx);
                return false;
            }
        } else {
            let (left, right) = self.accounts.split_at_mut(fi);
            let from_acc = &mut right[0];
            let to_acc = &mut left[ti];

            if from_acc.balance >= amount {
                from_acc.withdraw(amount);
                to_acc.deposit(amount);
                tx.approve();
                self.transactions.push(tx);
                return true;
            } else {
                self.transactions.push(tx);
                return false;
            }
        }
    }

}
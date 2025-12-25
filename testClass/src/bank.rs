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

}
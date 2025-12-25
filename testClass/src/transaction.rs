struct Transaction{
    from_accont:u32,
    to_account:u32,
    amount:f64,
    approved:bool,
}

impl Transaction{
    pub fn new(from_account:u32, to_account:u32, amount:f64)->Self{
        Self{
            from_account,
            to_account,
            amount,
            approved:false,
        }
    }

    pub fn approve(&mut self){
        self.approved = true;
        println!("Transaction from account {} to account {} for amount {} has been approved.", self.from_account, self.to_account, self.amount);
    }

    pub fn print(&self){
        println!("Transaction from account {} to account {} for amount {}. Approved: {}", self.from_account, self.to_account, self.amount, self.approved);
    }
}
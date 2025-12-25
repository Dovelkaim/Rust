use crate::costumer::Costumer;

#[derive(Clone)]

struct Account{
    number:i32,
    balance:f64,
    owner:Costumer,
}
impl Account{
    pub fn new(number:i32, balance:f64, owner:Costumer)->Self{
        Self{
            number,
            balance,
            owner,
        }
    }

    pub fn change_owner(&mut self, new_owner: Costumer) {
        self.owner = new_owner;
    }
    pub fn print(&self){
        println!("Account number: {}, balance: {}, owner: {:?}", self.number, self.balance, self.owner);
    }

    pub fn withdraw(&mut self, amout:f64)->bool{
        if amout > self.balance {
            println!("Insufficient funds");
            return false;
        } else {
            self.balance -= amout;
            println!("Withdrew {} from account number {}. New balance: {}", amout, self.number, self.balance);
            return true;
        }   
    }

    pub fn deposit(&mut self, amout:f64){
        self.balance+=amout;
        println!("Deposited {} to account number {}. New balance: {}", amout, self.number, self.balance);
    }
}
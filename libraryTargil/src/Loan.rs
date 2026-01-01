struct Loan{
    member_od:u32,
    isbn:u32,
    days:u32,
    approved:bool
}

impl Loan{
    pub fn new(member_id:u32,isbn:u32,days:u32,approved:bool)->Self{
        Self{
            member_od,
            isbn,
            days,
            approved
        }
    }
    pub fn approve(&mut self){
        self.approved=true;
        println!("Loan for member ID {} and book ISBN {} approved",self.member_id,self.isbn);
    }
    pub fn print(&self){
        println!("Loan - Member ID: {}, Book ISBN: {}, Days: {}, Approved: {}",self.member_id,self.isbn,self.days,self.approved);
    }
}
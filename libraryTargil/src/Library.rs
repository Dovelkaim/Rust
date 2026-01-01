struct Library{
    name:String,
    members: Vec<Member>,
    books: Vec<Book>,
    loans: Vec<Loan>,
}
impl Library{
    pub fn new(name:String)->Self{
        Self{
            name,
            members:Vec::new(),
            books:Vec::new(),
            loans:Vec::new(),
        }
    }
    pub fn add_member(&mut self, member:Member){
        self.members.push(member);
    }
    pub fn add_book(&mut self, book:Book){
        self.books.push(book);
    }
    pub fn add_loan(&mut self, loan:Loan){
        self.loans.push(loan);
    }
    pub fn print(&self){
        println!("Library name: {}", self.name);
        println!("Members:");
        for member in &self.members{
            member.print();
        }
        println!("Books:");
        for book in &self.books{
            book.print();
        }
        println!("Loans:");
        for loan in &self.loans{
            loan.print();
        }
    }
    pub fn find_members_index(&self, id:u32)->i32{

    for(index,member) in self.members.iter().enumerate(){
        if member.id==id{
            return index as i32;
        }
    }
    return -1;
    }

    pub fn find_books_index(&self, isbn:u32)->i32{
    for(index,books) in self.books.iter().enumerate(){
        if books.isbn==isbn{
            return index as i32;
        }
    }
    return -1;
    }
    pub fn borrow_book(&mut self, member_id: u32, isbn: u32, days: u32)->bool{


    }
    
}

#[derive(Clone)]
struct Book{
    isbn : u32,
    title:String,
    copies_total:u32,
    copies_available:u32,
}
impl Book{
    pub fn new(isbn:u32,title:String,copies_total:u32,copies_available:u32)->Self{
        Self{
            isbn,
            title,
            copies_total,
            copies_available,
        }
    }

    pub fn borrow_copies(&mut self,)->bool{
        if self.copies_available>0{
            self.copies_available-=1;
            println!("You have borrowed the book: {}",self.title);
            return true;
        }
        if slef.copies_available==0{
            println!("All copies are currently borrowed for the book: {}",self.title);
            return false;
        }
        println!("No copies available for the book: {}",self.title);
        false

    }

    pub fn return_copies(&mut self){
        if self.copies_total<self.copies_available+1{
            println!("Error: Cannot return more copies than total for the book: {}",self.title);
            return;
        }
        self.copies_available+=1;
        println!("You have returned the book: {}",self.title);
        return copies_available;

    }
    pub fn print(&self){
        println!("Book ISBN: {}, Title: {}, Total Copies: {}, Available Copies: {}",self.isbn,self.title,self.copies_total,self.copies_available);
    }
}
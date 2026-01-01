struct Member{
    name: String,
    id: u32,
    is_active:bool
}

impl Member{
pub fn new(name:String,id:u32)->Self{
    Self{
        name,
        id,
        is_active:true
    }
}

fn deactivate(&mut self){
    self.is_activate=false;
    println!("Member {} deactivated",self.name);
}
fn activate(&mut self){
    self.is_active=true;
    println!("Member {} activated",self.name);
}
fn print(&self){
    println!("Member name = {}, id = {}, is active = {}", self.name, self.id, self.is_active);
}

}
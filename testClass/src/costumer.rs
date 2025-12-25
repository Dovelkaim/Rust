#[derive(Debug, Clone)]
struct Costumer {
    id: i32,
    name: String,
    age: u8,
}

impl Costumer {
    pub fn new(id: i32, name:String, age: u8) -> Self {
        Self {
            id,
            name,
            age,
        }
    }

    pub fn print(&self) {
        println!(
            "the id of {} is {}, he is {} years old",
            self.name, self.id, self.age
        );
    }
}
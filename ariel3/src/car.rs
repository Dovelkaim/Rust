pub struct Car {
    pub color: String,
    pub model: String,
    pub year: u16,
    pub speed: u16,
}

impl Car {
    pub fn new(color: String, model: String, year: u16, speed: u16) -> Car {
        Car { color, model, year, speed }
    }

    pub fn speed_up(&mut self, vitesse: u16) {
        self.speed += vitesse;
        println!("The car speed is now: {} km/h", self.speed);
    }
    pub fn print(&mut self){
        println!("Car model: {}, color: {}, year: {}, speed: {} km/h",self.model, self.color, self.year, self.speed);
    }
}
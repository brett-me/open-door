fn main() {
    let mut bmw: Car = Car::new("bmw".into(), "m3".into(), 2025);
    bmw.drive();
    bmw.edit_model("C63".into());
    bmw.drive();
}

struct Car {
    make: String,
    model: String,
    year: u16,
}

impl Car {
    fn new(make: String, model: String, year: u16) -> Self {
        Self { make, model, year }
    }

    fn drive(&self) {
        println!("{} {} {} goes vroom", self.make, self.model, self.year);
    }

    fn edit_model(&mut self, update_model: String) {
        self.model = update_model;
    }
}

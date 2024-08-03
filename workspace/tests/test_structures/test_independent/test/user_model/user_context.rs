pub struct User {
    name: String,
    age: u8,
    weight: f32,
}

impl User {
    pub  fn new(name: String, age: u8, weight: f32) -> Self {
        User {
            name: name,
            age: age,
            weight: weight,
        }
    }

    pub  fn get_name(&self) -> &str {
        &self.name
    }

    pub  fn get_age(&self) -> u8 {
        self.age
    }

    pub  fn get_weight(&self) -> f32 {
        self.weight
    }

    pub  fn set_name(&mut self, new_name: String) {
        self.name = new_name
    }

    pub  fn set_age(&mut self, new_age: u8) {
        self.age = new_age
    }

    pub  fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight
    }
}

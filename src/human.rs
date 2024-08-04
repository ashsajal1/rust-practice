pub struct Human {
    pub name: String,
    pub age: i32,
}

impl Human {
    pub fn new(name: String, age: i32) -> Human {
        Human { name, age }
    }

    pub fn get_age(&self) -> i32 {
        self.age
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

mod child;

pub use child::Child;
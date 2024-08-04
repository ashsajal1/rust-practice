pub struct Child {
    pub name: String,
    pub age: i32,
}

impl Child {
    pub fn new(name: String, age: i32) -> Child {
        Child { name, age }
    }

    pub fn get_age(&self) -> i32 {
        self.age
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

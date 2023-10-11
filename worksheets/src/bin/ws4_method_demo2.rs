#[derive(Debug)]
struct Person {
    _name: String,
    age: u32,
}

impl Person {
    fn birth(_name: String) -> Self {
        Person {
            _name: _name,
            age: 0,
        }
    }

    fn grow_older(&mut self, years: u32) {
        self.age = self.age + years;
    }

    fn get_name(&self) -> &str {
        &self._name.as_str()
    }

    fn set_name(&mut self, new_name: &str) {
        self._name = String::from(new_name);
    }
}

fn main() {
    let mut p = Person::birth(String::from("alice"));
    p.grow_older(2); // implicit borrow of p
    Person::grow_older(&mut p, 3); // explicit borrow of p
    println!("{p:?}");
    println!("{}", p.get_name());

    p.set_name("Barbara");
    println!("{}", p.get_name());
}

#[derive(Debug)]
struct Person {
    _name: String,
    age: u32,
}

impl Person {
    fn birth(name: String) -> Self {
        Person {
            _name: name,
            age: 0,
        }
    }

    fn grow_older(&mut self) {
        self.age += 1;
    }
}

fn main() {
    let mut p1 = Person::birth(String::from("alice"));
    let mut p2 = Person {
        _name: String::from("bob"),
        age: 20,
    };
    p1.grow_older();
    p2.grow_older();
    println!("{p1:?}");
    println!("{p2:?}");
}

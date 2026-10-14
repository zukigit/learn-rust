use std::fmt::{self};

struct Person {
    age: u8
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "this is struct's age {}", self.age)
    }
}

fn main() {
    let person = Person{age: 20};

    // we cant call person directly without impl Display
    println!("this is person string {}", person)
}

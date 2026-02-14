#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

// unit struct
struct Unit;

// tuple struct
struct Pair(i32, u8);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

pub fn structures() {
    let person = Person {
        age: 25,
        name: String::from("zuki"),
    };

    // making struct using existed struct
    let sec_person = Person {name: String::from("wai"), ..person};

    println!("person: {:?}", person);
    println!("person.name: {}", person.name);
    println!("person.age: {}", person.age);

    println!("sec_person.age {}", sec_person.age);

    let Person{name: sec_person_name, age: sec_person_age} = sec_person;

    println!("sec_person_name: {}", sec_person_name);
    println!("sec_person_age: {}", sec_person_age);

    let _unit = Unit;

    let pair = Pair(12, 1);
    println!("pair.0: {}", pair.0);
    println!("pair.1: {}", pair.1)
}
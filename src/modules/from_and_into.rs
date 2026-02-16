#[derive(Debug)]
struct NumberStore {
    owner: String,
    number: i32
}

impl From<i32> for NumberStore {
    fn from(value: i32) -> Self {
       NumberStore { owner: "default_owner".to_string(), number: value } 
    }
}

#[derive(Debug)]
struct NumberStore1 {
    owner: String,
    number: i32
}

impl Into<NumberStore1> for i32 {
    fn into(self) -> NumberStore1 {
        NumberStore1 { owner: "default_owner".to_string(), number: self } 
    }
}

pub fn from_and_into() {
    let store = NumberStore::from(32);
    println!("store.owner: {}", store.owner);
    println!("store.number: {}", store.number);
    println!("store: {:?}", store);

    let int = 5;
    let store1: NumberStore1 = int.into();
    println!("store.owner: {}", store1.owner);
    println!("store.number: {}", store1.number);
    println!("store: {:?}", store1);

}
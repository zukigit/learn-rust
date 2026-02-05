#[allow(unused_variables)]
pub fn variables() {
    let mut mutable = 23;
    let pointer = &mut mutable;

    *pointer = 11;

    println!("mutable: {}", mutable);

    let logical: bool = false;
    let a_float: f64 = 1.1;
    let an_int = 123;

    let array = [1, 2, 4];
}
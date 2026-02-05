pub fn tuples() {
    let long_tuples: (i32, u32, bool, &str, char) = (123, 111u32, true, "it is tuple", 'c');

    println!("long_tuples.0: {:?}", long_tuples.0);
    println!("long_tuples.1: {:?}", long_tuples.1);

    let tuple_of_tuples = ((1, 2, 3), "hsf", (true, false));
    println!("tuple_of_tuples.0.1: {:?}", tuple_of_tuples.0.1);

    println!("tuple_of_tuples: {:?}", tuple_of_tuples);

    let (a, b, c) = tuple_of_tuples;
    println!("{:?} {b} {:?}", a, c);
}
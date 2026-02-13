use std::mem;

#[allow(unused_variables)]
pub fn arrays_and_slices() {
    let array_a = [1, 2, 3];
    let array_b: [i32; 100] = [1; 100]; // [1, 1, 1, 1, ..]

    println!("array_b.len(): {}", array_b.len());
    println!("mem::size_of_val(&array_b): {} bytes", mem::size_of_val(&array_b));

    println!("&array_b[1 .. 2]: {:?}", &array_b[1 .. 2]); // -> ok
    // println!("array_b[1 .. 2]: {:?}", array_b[1 .. 2]) // -> not ok

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    let mut vet: Vec<&str> = Vec::new();

    vet.push("10");
    vet.push("hello");

    assert_eq!(vet.capacity(), 0);
}
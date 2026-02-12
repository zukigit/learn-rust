fn reverse(pair: (&str, i32)) -> (i32, &str){
    // Tuples can be destructured to create bindings.
    let (string_param, int_param) = pair;

    (int_param, string_param)
}

pub fn tuples() {
    // A tuple with a bunch of different types.
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);

    println!("long_tuple.0: {}", long_tuple.0);
    println!("long_tuple.0: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple_of_tuples: {:?}", tuple_of_tuples);

    let pair = ("zuki", 25);

    println!("pair: {:?}", pair);
    println!("reverse(pair): {:?}", reverse(pair))
}
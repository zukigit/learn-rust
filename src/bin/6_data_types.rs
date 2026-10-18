fn main() {
    // scalar types
    let x: u8 = 255;

    // 255 + 2 -> went back to the start
    let (result, overflowed) = x.overflowing_add(2);
    println!("result: {}, overflowed: {}", result, overflowed);

    // the fllowing data type sizes depend on the os
    println!("isize::MIN = {}", isize::MIN);
    println!("isize::MAX = {}", isize::MAX);
    println!("usize::MIN = {}", usize::MIN);
    println!("usize::MAX = {}", usize::MAX);

    // compund types
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The values of x, y, z are: {x} {y} {z}");
    println!("The values of x, y, z are: {} {} {}", tup.0, tup.1, tup.2);

    // array
    let my_array = [1, 2, 3, 4, 5];
    let my_value = my_array[6];

    println!("my_value is {}", my_value);
}

fn main() {
    let is_male = true;
    let is_male_string = if is_male{ "it is male" } else { "it is female" };

    println!("is_male_string: {}", is_male_string);

    // a loop can return value and that is cool
    let mut counter = 0;
    let loop_result = loop {
        counter += 1;

        if counter == 2 {
            break counter
        }
    };
    println!("final loop result: {}", loop_result);
}

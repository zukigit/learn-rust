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

    // inner loop can break or continue outer loop
    let mut i = 0;
    'outer: loop {
        i += 1;
        let mut j = 0;

        loop {
            j += 1;

            if j == 3 {
                continue 'outer; // skip the rest of inner loop, go to next i
            }
            if i == 4 {
                break 'outer; // exit both loops entirely
            }

            println!("i = {i}, j = {j}");
        }
    }

    // loop through a collection
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // match
    let number = 7;
    match number {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4..=6 => println!("four to six"),
        n if n % 2 == 1 => println!("{n} is odd and 7 or more"),
        _ => println!("something else"),
    }
}

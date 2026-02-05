#[derive(Debug)]
struct Structure();

pub fn debug() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);

    // `Structure` is printable because of debug
    println!("{:?} is Structure", Structure());
}
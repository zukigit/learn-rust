#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

pub fn debug() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);

    // `Structure` is printable!
    println!("{:?} is Structure", Structure(1));
    println!("{:?} is Deep", Deep(Structure(1)))
}
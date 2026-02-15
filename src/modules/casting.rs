pub fn casting() {
     let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;
    let str = decimal.to_string();

    println!("Casting: {} -> {} -> {} -> {}", decimal, integer, character, str);
}
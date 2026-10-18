const MY_CONST_VAR: u8 = 192;
static MY_STATIC_VAR: u8 = 192;

fn main() {
    let immuted_var = 6;
    println!("This is the immuted variable {}", immuted_var);

    let mut muted_var = 10;
    println!(
        "This is the muatable variable before reassign {}",
        muted_var
    );

    muted_var = 23;
    println!("This is the muatable variable after reassign {}", muted_var);

    println!("This is the const {}", MY_CONST_VAR);
    println!("This is the static const {}", MY_STATIC_VAR); // static constant livs in memory

    let immuted_var = 7;
    println!("imutable varible after shadowing {}", immuted_var);
}

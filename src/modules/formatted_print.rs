pub fn formatted_print() {
    println!("{} days", 10);
    println!("{0} lives in {1}. {2} lives in {1} too", "zuki", "Rangoon", "wai");

    println!("{sub} {verb} {obj}",
        sub="he",
        verb="is",
        obj="good");
    
    println!("{number:0>width$}", number=1, width=5);
    println!("{number:0>width$}", number=1, width=5);
}
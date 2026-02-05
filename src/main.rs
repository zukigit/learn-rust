mod modules;

fn main() {
    modules::hello_world::hello_world();
    modules::formatted_print::formatted_print();
    modules::debug::debug();
    say_hello!();
    modules::operators::operators();
    modules::tuples::tuples();
}

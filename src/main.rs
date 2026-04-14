use std::env::args;

fn main() {
    for argument in args() {
        println!("{argument}");
    }
}

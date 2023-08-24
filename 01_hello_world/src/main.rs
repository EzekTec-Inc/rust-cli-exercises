//!   The simplest of simple for Rust programs!

mod hello;

use hello::say_hello;

fn main() {
    println!("{}", say_hello().unwrap());
}

#[cfg(test)]
mod main_runs {

    use crate::main;

    #[test]
    fn returns_nothing() {
        let result = main();
        assert!(result == ());
    }
}

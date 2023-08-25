mod func1;
use crate::func1::fn1;

mod func2;
use crate::func2::fn2;

mod mod_nested {
    pub(super) mod func3; // loads mod_nested/func3.rs
}
use crate::mod_nested::func3::fn3;
fn main() {
    let sum = fn1() + fn2() + fn3();

    println!("{}", sum);
}

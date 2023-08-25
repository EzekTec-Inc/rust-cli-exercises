mod derive_style;
use crate::derive_style::read_args as read_args_derive_style;

fn main() {
    let args_d = read_args_derive_style();

    println!(
        "arg1: {}, arg2: {:?}, flag: {:?}",
        args_d.arg1, args_d.arg2, args_d.flag
    );
}

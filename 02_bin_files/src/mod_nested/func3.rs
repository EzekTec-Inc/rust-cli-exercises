pub fn fn3() -> u8 {
    3
}

#[cfg(test)]
mod fn3_tests {

    use crate::mod_nested::func3::fn3;

    #[test]
    fn return_3() {
        assert_eq!(fn3(), 3);
    }
}

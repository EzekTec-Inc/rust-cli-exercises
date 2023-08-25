pub fn fn1() -> u8 {
    1
}

#[cfg(test)]
mod fn1_tests {

    use super::*;

    #[test]
    fn return_1() {
        assert_eq!(fn1(), 1);
    }
}

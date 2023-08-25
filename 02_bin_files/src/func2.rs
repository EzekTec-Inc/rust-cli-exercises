use crate::func1::fn1;

pub fn fn2() -> u8 {
    2 * fn1()
}

#[cfg(test)]
mod fn2_tests {

    use crate::func2::fn2;

    #[test]
    fn return_2() {
        assert_eq!(fn2(), 2);
    }
}

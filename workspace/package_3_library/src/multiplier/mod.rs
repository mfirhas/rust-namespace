pub fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

#[cfg(test)]
pub mod multiplier_test {
    use crate::multiplier::multiply;

    #[test]
    fn test_multiply() {
        let a = 1;
        let b = 2;
        let expected = 2;
        let result = multiply(a, b);
        assert_eq!(expected, result);
    }
}

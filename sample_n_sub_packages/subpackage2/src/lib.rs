pub fn mul(left: usize, right: usize) -> usize {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = mul(2, 2);
        assert_eq!(result, 4);
    }
}

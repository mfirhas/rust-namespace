#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod adder;
mod adder_test;

pub mod multiplier;

pub mod subtractor {
    pub fn subtract(a: i32, b: i32) -> i32 {
        return a - b;
    }

    #[test]
    fn test_subtract() {
        let a = 1;
        let b = 2;
        let expected = -1;
        let result = subtract(a, b);
        assert_eq!(expected, result);
    }
}

pub mod divider {
    pub fn divide(a: f32, b: f32) -> f32 {
        return a / b;
    }
}
mod divider_test {
    #[test]
    fn test_subtract() {
        let a = 1 as f32;
        let b = 2 as f32;
        let expected = 0.5;
        let result = crate::divider::divide(a, b);
        assert_eq!(expected, result);
    }
}

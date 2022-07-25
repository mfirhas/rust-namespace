#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod inline_mod {
    fn print_me() {
        println!("im function from inline mod")
    }
}

// no need to declare `pub` as it's in the same level/module (sibling)
pub mod inline {
    pub const CONSTANT: &str = "string constant";
}

// no need to declare `pub` as it's in the same level/module (sibling)
pub mod inline_fn {
    pub fn add(a: i32, b: i32) -> i32{
        a + b
    }
}

pub mod file1;
pub mod file2;
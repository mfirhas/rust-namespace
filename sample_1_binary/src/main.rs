// no need to declare `pub` as it's in the same level/module (sibling)
mod inline {
    pub const CONSTANT: &str = "string constant";
}

// no need to declare `pub` as it's in the same level/module (sibling)
mod inline_fn {
    pub fn add(a: i32, b: i32) -> i32{
        a + b
    }
}

pub mod file1;
pub mod file2;

fn main() {
    println!("Hello, world!");
    println!("constant: {}", inline::CONSTANT);
    println!("return: {}", inline_fn::add(7, 5));
    file1::print_me();
    file1::sub_file_1::sub_sub_file_1::print_me_from_outside();
    file2::print_me();
}

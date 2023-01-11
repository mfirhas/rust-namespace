
pub mod lib1;
pub mod lib2;

pub const C: &str = "test";

pub fn get() -> String {
    println!("{}", lib1::LIB1_CONST);
    String::from("hello")
}

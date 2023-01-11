// pub mod asd; ////! cannot do this as submodule must be defined from within abother folder, same level crate can only be declared inside main.rs

pub mod anu {
    pub fn a() {
        println!("A");
    }
}
pub const CONSTANT: &str = "ANU";
const CONSTANT2: i32 = 123;

pub fn b() {
    // sample_simple::b(); ////! cannot do this because package level import can only be imported from main.rs
    println!("B");
}
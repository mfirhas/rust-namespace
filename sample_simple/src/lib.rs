// pub mod asd; ////! if main, lib and this module are all in same level, then this module can only be imported from main crate.


// use crate:: ////! only can import things inside this file only because the main crate already exists and other files can only be imported there.

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
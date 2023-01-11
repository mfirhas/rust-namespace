pub mod lib;
pub mod asd;

pub fn from_main() {
    println!("im from main");
}

fn main() {
    lib::b(); // can import b function from lib declaration, or
    sample_simple::b(); // can import b function from package's name
    println!("{}", lib::CONSTANT);
    println!("{}", sample_simple::CONSTANT);
    // sample_simple::C:  ////! cannot use constant C from asd.rs this as asd.rs is not convention for library crate's file name
    // sample_simple::get() ////! cannot use function get() from asd.rs as asd.rs is not convention for library crate's file name
    println!("{}", asd::C);
    println!("Hello, world!");
}

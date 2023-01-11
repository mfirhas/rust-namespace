pub mod lib;

fn main() {
    lib::b();
    sample_simple::b();
    println!("{}", lib::CONSTANT);
    println!("{}", sample_simple::CONSTANT);
    println!("Hello, world!");
}

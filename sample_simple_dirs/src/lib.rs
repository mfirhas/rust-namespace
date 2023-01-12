// because
pub mod asd;

pub mod a {
    use super::*;
    pub fn func() {
        asd::sdf::function();
        println!("test");
    }
}
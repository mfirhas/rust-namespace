pub mod sub_file_1;
mod sub_file_1_2;

pub fn print_me() {
    println!("im from file 1");
    sub_file_1::print_me();
    sub_file_1_2::print_me();
}
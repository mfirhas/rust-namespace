pub mod sub_sub_file_1;

pub const FILE1: &str = "im constant from file1";

pub fn print_me() {
    println!("-im from file1 submodule");
    sub_sub_file_1::print_me();
}
pub mod sub_mod;

pub const FILE2: &str = "im constant from file2";

pub mod file2_submodule {
    pub fn print_me() {
        println!("-im submodule of file2");
    }
}

pub fn print_me() {
    println!("im from file 2");
    println!("im constant from file 1: {}", crate::file1::sub_file_1::FILE1);
    file2_submodule::print_me();
    sub_mod::print_me();
}

use library::{inline_fn, funcs};

fn main() {
    println!("im main function in binary crate named binary");
    inline_fn::print_me();
    funcs::do_me();
}
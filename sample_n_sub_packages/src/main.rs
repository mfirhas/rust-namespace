use subpackage1::add;
use subpackage2::mul;

fn main() {
    let ret = add(1, 2);
    println!("{}", ret);
    let ret = mul(3 , 5);
    println!("{}", ret);
    println!("Hello, world!");
}

mod service;

fn main() {
    println!("im package_1_binary");
    println!("addition result: {}", service::add(1, 2));
    println!("multiplication result: {}", service::multiply(1, 2));
}

mod service;
fn main() {
    println!("im package_2_binary");
    println!("subtraction result: {}", service::subtract(1, 2));
    println!("division result: {}", service::divide(1 as f32, 2 as f32));
}

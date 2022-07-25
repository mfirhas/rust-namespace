use package_3_library::{adder, multiplier};

pub fn add(a: i32, b: i32) -> i32 {
    adder::add(a, b)
}

pub fn multiply(a: i32, b: i32) -> i32 {
    return multiplier::multiply(a, b);
}

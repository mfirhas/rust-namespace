use package_3_library::{divider, subtractor};

pub fn subtract(a: i32, b: i32) -> i32 {
    subtractor::subtract(a, b)
}

pub fn divide(a: f32, b: f32) -> f32 {
    return divider::divide(a, b);
}

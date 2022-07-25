#[cfg(test)]
use crate::adder;

#[test]
fn test_add() {
    let a = 1;
    let b = 2;
    let expected = 3;
    let result = adder::add(a, b);
    assert_eq!(expected, result);
}

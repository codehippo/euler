use num::{BigUint, range_inclusive};
use std::iter::FromIterator;

fn main() {
    let expanded_factorial:Vec<BigUint> = Vec::from_iter(range_inclusive(BigUint::from(2_u32), BigUint::from(100_u32)));
    let result: BigUint = expanded_factorial.iter().product();

    println!("{}", result.to_str_radix(10).chars().map(|i| i.to_digit(10).unwrap()).sum::<u32>());
}

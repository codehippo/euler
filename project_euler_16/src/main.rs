use num::BigUint;

fn main() {
    let number = BigUint::from(2_u32).pow(1000);

    println!("{}", number.to_str_radix(10).chars().map(|i| i.to_digit(10).unwrap()).sum::<u32>());
}

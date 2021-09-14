use num_bigint::BigUint;
use num_traits::identities::One;

struct FibonacciPair {
    current: BigUint,
    previous: BigUint,
}

impl FibonacciPair {
    fn new(current: BigUint, previous: BigUint) -> Self {
        FibonacciPair {
            current,
            previous
        }
    }

    fn first() -> Self {
        FibonacciPair {
            current: BigUint::one(),
            previous: BigUint::one()
        }
    }
}

impl Iterator for FibonacciPair {
    type Item = FibonacciPair;

    fn next(&mut self) -> Option<Self::Item> {
        let new_current = self.current.clone() + self.previous.clone();
        let new_previous = self.current.clone();

        self.current = new_current.clone();
        self.previous = new_previous.clone();

        Some(FibonacciPair::new(new_current, new_previous))
    }
}

fn main() {
    let first_fibonacci_pair = FibonacciPair::first();

    let largest = first_fibonacci_pair
        .enumerate()
        .find(|x|
            x.1.current.to_str_radix(10).chars().count() == 1000
        )
        .unwrap().0;

    println!("It is: {}", largest + 1 + 2);
}

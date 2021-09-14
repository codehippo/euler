#[derive(Clone, Copy)]
struct PrimeFactorWithMultiplicity {
    prime: u64,
    multiplicity: u64
}

impl PrimeFactorWithMultiplicity {
    fn new(prime: u64, multiplicity: u64) -> Self {
        PrimeFactorWithMultiplicity {prime, multiplicity}
    }

    fn increase_multiplicity(&mut self) {
        self.multiplicity += 1;
    }
}

struct PrimeFactorisedNumberWithMultiplicity {
    num: u64,
    prime_factors_with_multiplicity: Vec<PrimeFactorWithMultiplicity>,
    factors: u64
}

impl PrimeFactorisedNumberWithMultiplicity {
    fn new(num: u64) -> Self {
        let mut result:Vec<PrimeFactorWithMultiplicity> = Vec::new();
        let mut tmp_num = num;

        while tmp_num != 1 {
            for test_num in 2..=tmp_num {
                if tmp_num % test_num == 0 {
                    if result.is_empty() {
                        result.push(PrimeFactorWithMultiplicity::new(test_num, 1));
                    } else if result.iter().find(|x| x.prime == test_num).is_none() {
                        result.push(PrimeFactorWithMultiplicity::new(test_num, 1));
                    } else {
                        result.iter_mut().find(|x| x.prime == test_num).unwrap().increase_multiplicity();
                    }

                    tmp_num /= test_num;

                    break;
                }
            }
        }

        PrimeFactorisedNumberWithMultiplicity {
            num,
            factors: number_of_divisors(result.clone()),
            prime_factors_with_multiplicity: result
        }
    }
}

fn number_of_divisors(prime_factors_with_multiplicity: Vec<PrimeFactorWithMultiplicity>) -> u64 {
    prime_factors_with_multiplicity.iter().map(|x| x.multiplicity + 1).product()
}

fn main() {
    println!("{}", (1..)
        .map(|x| PrimeFactorisedNumberWithMultiplicity::new(x * (x + 1) / 2 as u64))
        .find(|x| x.factors > 500).unwrap().num);
}

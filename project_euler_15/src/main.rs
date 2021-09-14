use std::collections::HashSet;
use std::iter::FromIterator;
use std::cmp;

struct PrimeFactorisedNumberWithMultiplicity {
    num: u32,
    prime_factors_with_multiplicity: Vec<PrimeFactorWithMultiplicity>
}

impl PrimeFactorisedNumberWithMultiplicity {
    fn new(num: u32) -> Self {
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

        PrimeFactorisedNumberWithMultiplicity {num, prime_factors_with_multiplicity:result}
    }

    fn multiplicity_of_prime(&self, test_prime: u32) -> u32 {
        let result = self.prime_factors_with_multiplicity.iter().find(|x| x.prime == test_prime);

        match result {
            Some(x) => x.multiplicity,
            None => 0
        }
    }
}

#[derive(Clone)]
struct Ratio<T: Clone> {
    numer: T,
    denom: T
}

impl<T: Clone> Ratio<T> {
    fn new(numer: T, denom: T) -> Self {
        Ratio {numer, denom}
    }
}

struct Product<T> {
    elements: Vec<T>
}

impl std::ops::Mul for Product<u32> {
    type Output = Product<u32>;

    fn mul(self, other: Product<u32>) -> Product<u32> {
        Product::new(self.elements.iter().chain(&other.elements).cloned().collect())
    }
}

#[derive(Clone, Copy)]
struct PrimeFactorWithMultiplicity {
    prime: u32,
    multiplicity: u32
}

impl PrimeFactorWithMultiplicity {
    fn new(prime: u32, multiplicity: u32) -> Self {
        PrimeFactorWithMultiplicity {prime, multiplicity}
    }

    fn increase_multiplicity(&mut self) {
        self.multiplicity += 1;
    }
}

impl PartialEq for PrimeFactorWithMultiplicity {
    fn eq(&self, other: &Self) -> bool {
        self.prime == other.prime
    }
}

impl Product<PrimeFactorWithMultiplicity> {
    fn from_num_product(product: Product<u32>) -> Self {
        let mut result_elements:Vec<PrimeFactorWithMultiplicity> = Vec::new();
        let mut tmp_prime_factorised_numbers:Vec<PrimeFactorisedNumberWithMultiplicity> = Vec::new();

        for num in product.elements {
            tmp_prime_factorised_numbers.push(PrimeFactorisedNumberWithMultiplicity::new(num));
        }

        let all_primes:HashSet<_> = HashSet::from_iter(
            tmp_prime_factorised_numbers
            .iter()
            .clone()
            .map(|x| x.prime_factors_with_multiplicity.clone())
            .flatten()
            .map(|x| x.prime)
        );

        for prime in all_primes {
            let multiplicity = tmp_prime_factorised_numbers.iter().clone().map(|x| x.multiplicity_of_prime(prime)).sum();

            result_elements.push(PrimeFactorWithMultiplicity::new(prime, multiplicity));
        }

        Product::new(result_elements)
    }

    fn divide_by(&self, other:Product<PrimeFactorWithMultiplicity>) -> Ratio<Product<PrimeFactorWithMultiplicity>> {
        Ratio::new(
            self.clone(),
            other.clone()
        )
    }
}

impl Product<u32> {
    fn into_prime_factorised_product(self) -> Product<PrimeFactorWithMultiplicity> {
        Product::from_num_product(self)
    }
}

impl Ratio<Product<PrimeFactorWithMultiplicity>> {
    fn reduced(&mut self) {
        for mut denom_prime_factor in &mut self.denom.elements {
            let tmp_numer_prime_factor = self.numer.elements.iter_mut().find(|&&mut x| x == *denom_prime_factor);

            if tmp_numer_prime_factor.is_some() {
                let numer_prime_factor = tmp_numer_prime_factor.unwrap();

                let denom_factor_multiplicity:i64 = denom_prime_factor.multiplicity as i64;
                let numer_factor_multiplicity:i64 = numer_prime_factor.multiplicity as i64;

                denom_prime_factor.multiplicity = cmp::max(0, denom_factor_multiplicity - numer_factor_multiplicity) as u32;
                numer_prime_factor.multiplicity = cmp::max(0, numer_factor_multiplicity - denom_factor_multiplicity) as u32;
            }
        };

        self.denom.elements.retain(|&x| x.multiplicity != 0);
        self.numer.elements.retain(|&x| x.multiplicity != 0);
    }

    fn calculate(&self) -> u128 {
        self.numer.elements.iter().map(|x| x.prime.pow(x.multiplicity) as u128).product::<u128>()
    }
}

impl<T> Product<T> {
    fn new(elements: Vec<T>) -> Self {
        Product {elements}
    }
}

impl<T: Clone> Clone for Product<T> {
    fn clone(&self) -> Product<T> {
        Product::new(self.elements.iter().cloned().collect())
    }
}

struct Factorial {
    num: u32
}

impl Factorial {
    fn new(num: u32) -> Self {
        Factorial {num}
    }

    fn to_product(&self) -> Product<u32> {
        Product::new((2..=self.num).collect())
    }
}

fn main() {
    let numerator = Factorial::new(40).to_product().into_prime_factorised_product();
    let denominator = (Factorial::new(20).to_product() * Factorial::new(20).to_product()).into_prime_factorised_product();

    let mut division = numerator.divide_by(denominator);
    division.reduced();

    println!("Result: {}", division.calculate());
}

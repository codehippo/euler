use std::collections::HashSet;

const MAX: u32 = 10_000;

#[derive(Eq, PartialEq, Hash, Debug)]
struct NumberWithDivisors {
    num: u32,
    divisors: Vec<u32>,
    sum_of_divisors: u32
}

impl NumberWithDivisors {
    fn new(num: u32) -> Self {
        let divisors = Self::find_divisors(&num);
        let sum_of_divisors = Self::sum_of_divisors(&divisors);

        NumberWithDivisors {
            num,
            divisors,
            sum_of_divisors
        }
    }

    fn find_divisors(num: &u32) -> Vec<u32> {
        let mut output_vec:Vec<u32> = vec![];

        (1_u32..*num).for_each(
            |x| {
                if num % x == 0 {
                    output_vec.push(x);
                }
            }
        );

        output_vec
    }

    fn sum_of_divisors(divisors: &Vec<u32>) -> u32 {
        divisors.iter().sum()
    }
}

struct AmicablePair<'a> {
    a: &'a NumberWithDivisors,
    b: &'a NumberWithDivisors
}

impl<'a> AmicablePair<'a> {
    fn new(a: &'a NumberWithDivisors, b: &'a NumberWithDivisors) -> Self {
        AmicablePair {
            a,
            b
        }
    }
}

struct AmicablePairStorage<'a> {
    vector: Vec<AmicablePair<'a>>
}

impl AmicablePairStorage<'_> {
    fn new() -> Self {
        AmicablePairStorage {
            vector: Vec::new()
        }
    }

    fn sum(&self) -> u32 {
        let mut set_of_unique_amicable_numbers:HashSet<&NumberWithDivisors> = HashSet::new();

        for amicable_pair in &self.vector {
            set_of_unique_amicable_numbers.insert(&amicable_pair.a);
            set_of_unique_amicable_numbers.insert(&amicable_pair.b);
        }

        set_of_unique_amicable_numbers.iter().map(|x| x.num).sum()
    }
}

struct NumbersWithDivisorsCache {
    cache: HashSet<NumberWithDivisors>,
}

impl<'a: 'b, 'b> NumbersWithDivisorsCache {
    fn new() -> Self {
        NumbersWithDivisorsCache {
            cache: HashSet::new()
        }
    }

    fn pair_up_and_return_sum(&'a self, amicable_pair_storage: &'b mut AmicablePairStorage<'b>) -> u32 {
        self.cache.iter()
            .for_each(|x| {
                let potential_partner = self.cache.iter()
                    .find(|y| {
                        y.num == x.sum_of_divisors && y.sum_of_divisors == x.num && y.num != x.num
                    });

                match potential_partner {
                    Some(y) => amicable_pair_storage.vector.push(AmicablePair::new(x, y)),
                    None => (),
                }
            });

        amicable_pair_storage.sum()
    }
}

fn main() {
    let mut amicable_pair_storage = AmicablePairStorage::new();
    let mut numbers_cache = NumbersWithDivisorsCache::new();

    (2_u32..MAX)
        .map(|x| NumberWithDivisors::new(x))
        .filter(|x| x.sum_of_divisors < MAX)
        .for_each(|x| {
            let _ = numbers_cache.cache.insert(x);
        });

    let sum = numbers_cache.pair_up_and_return_sum(&mut amicable_pair_storage);

    println!("Result: {:#?}", sum);
}

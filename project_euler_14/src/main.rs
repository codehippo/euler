use std::collections::HashSet;
use std::hash::{Hash, Hasher};

struct CollatzNumberCache {
    set: HashSet<CollatzNumber>
}

impl CollatzNumberCache {
    fn new() -> Self {
        let mut new_collatz_cache = CollatzNumberCache {
            set: HashSet::new()
        };

        new_collatz_cache.set.insert(CollatzNumber::new_complete(1, 1));

        new_collatz_cache
    }
}

#[derive(Copy, Clone)]
struct CollatzNumber {
    value: u64,
    length: Option<u64>,
}

impl CollatzNumber {
    fn new(value: u64) -> Self {
        CollatzNumber {
            value,
            length: None,
        }
    }

    fn new_complete(value: u64, length: u64) -> Self {
        CollatzNumber {
            value,
            length: Some(length)
        }
    }
}

impl PartialEq for CollatzNumber {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for CollatzNumber {}

impl Hash for CollatzNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

struct CollatzNumberIterator<'a> {
    current_collatz_number: CollatzNumber,
    cache: &'a mut CollatzNumberCache,
    local_chain: Vec<CollatzNumber>,
}

impl<'a> CollatzNumberIterator<'a> {
    fn new(starting_collatz_number: CollatzNumber, cache: &'a mut CollatzNumberCache) -> Self {
        CollatzNumberIterator {
            current_collatz_number: starting_collatz_number,
            cache,
            local_chain: vec!(starting_collatz_number)
        }
    }

    fn compute_lengths(&mut self, linking_collatz_number: CollatzNumber) {
        let source_collatz_number = self.cache.set.get(&linking_collatz_number).unwrap();
        for (i, mut collatz_number) in self.local_chain.iter_mut().rev().enumerate() {
            collatz_number.length = Some((i as u64 + 1) + source_collatz_number.length.unwrap());
        }
    }

    fn transfer_numbers_to_lookup_array(&mut self) {
        for collatz_number in &self.local_chain {
            self.cache.set.insert(*collatz_number);
        }
    }
}

impl<'a> Iterator for CollatzNumberIterator<'a> {
    type Item = CollatzNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let value_to_set: u64;

        if self.current_collatz_number.value % 2 == 0 {
            value_to_set = self.current_collatz_number.value / 2;
        } else {
            value_to_set = self.current_collatz_number.value * 3 + 1;
        }

        let potential_next_collatz_number = CollatzNumber::new(value_to_set);

        if self.cache.set.contains(&potential_next_collatz_number) {
            self.compute_lengths(potential_next_collatz_number);
            self.transfer_numbers_to_lookup_array();

            None
        } else {
            let next_collatz_number = potential_next_collatz_number;
            self.local_chain.push(next_collatz_number);
            self.current_collatz_number = next_collatz_number;

            Some(next_collatz_number)
        }
    }
}

fn main() {
    let mut cache = CollatzNumberCache::new();

    for idx in 2..1_000_000 {
        let starting_number = CollatzNumber::new(idx);
        let collatz_iterator = CollatzNumberIterator::new(starting_number, &mut cache);

        collatz_iterator.last();
    }

    let biggest = cache.set.iter().max_by(|&a, &b| a.length.unwrap().cmp(&b.length.unwrap())).unwrap().value;
    println!("Number producing the longest chain is: {}", biggest);
}

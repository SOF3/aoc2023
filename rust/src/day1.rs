use std::{array, iter, mem};

#[aoc_runner_derive::aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            fn first_digit_in_iter(iter: impl Iterator<Item = char>) -> u32 {
                iter.filter_map(|digit| digit.to_digit(10)).next().expect("line has no digits")
            }

            let first = first_digit_in_iter(line.chars());
            let last = first_digit_in_iter(line.chars().rev());
            first * 10 + last
        })
        .sum()
}

#[aoc_runner_derive::aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    struct Trie {
        children: [Option<Box<Trie>>; 36],
        value:    Option<u32>,
    }
    impl Default for Trie {
        fn default() -> Self { Self { children: array::from_fn(|_| None), value: None } }
    }
    impl Trie {
        fn insert(&mut self, mut chars: impl Iterator<Item = char>, value: u32) {
            if let Some(first) = chars.next() {
                let alpha = first.to_digit(36).unwrap();
                self.children[alpha as usize].get_or_insert_with(Box::default).insert(chars, value);
            } else {
                self.value = Some(value)
            }
        }

        fn lookup(&self, ch: char) -> Option<&Trie> {
            let alpha = ch.to_digit(36).unwrap();
            self.children[alpha as usize].as_deref()
        }
    }

    const DIGITS: &[(&str, u32)] = &[
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut trie = Trie::default();
    let mut rev_trie = Trie::default();

    for &(name, value) in DIGITS {
        trie.insert(name.chars(), value);
        rev_trie.insert(name.chars().rev(), value);
    }

    let mut active_tries = Vec::with_capacity(9);
    let mut active_tries_swap = Vec::with_capacity(9);

    input
        .lines()
        .map(|line| {
            fn find_digit<'t>(
                active_tries: &mut Vec<&'t Trie>,
                active_tries_swap: &mut Vec<&'t Trie>,
                iter: impl Iterator<Item = char>,
                base_trie: &'t Trie,
            ) -> u32 {
                active_tries.clear();
                for ch in iter {
                    if let Some(digit) = ch.to_digit(10) {
                        return digit;
                    }

                    active_tries_swap.clear();
                    for trie in active_tries.iter().copied().chain(iter::once(base_trie)) {
                        if let Some(child) = trie.lookup(ch) {
                            if let Some(value) = child.value {
                                return value;
                            }

                            active_tries_swap.push(child);
                        }
                    }
                    mem::swap(active_tries, active_tries_swap);
                }

                panic!("no digit in line")
            }

            let first = find_digit(&mut active_tries, &mut active_tries_swap, line.chars(), &trie);
            let last = find_digit(
                &mut active_tries,
                &mut active_tries_swap,
                line.chars().rev(),
                &rev_trie,
            );
            first * 10 + last
        })
        .sum()
}

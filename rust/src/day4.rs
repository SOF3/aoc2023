use std::collections::HashSet;

struct Line<L, R> {
    left:  L,
    right: R,
}

impl<L, R> Line<L, R>
where
    L: FromIterator<u32>,
    R: FromIterator<u32>,
{
    fn parse(mut line: &str) -> Self {
        line = line.strip_prefix("Card").expect("start with card");
        line = line.trim_start();
        let (_card, rest) = line.split_once(':').unwrap();
        let (left, right) = rest.split_once('|').unwrap();
        Self {
            left:  left
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect(),
            right: right
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect(),
        }
    }
}

#[aoc_runner_derive::aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line = Line::<HashSet<u32>, Vec<u32>>::parse(line);
            let matches = line.right.iter().filter(|&v| line.left.contains(v)).count();
            if matches > 0 {
                1 << (matches - 1)
            } else {
                0
            }
        })
        .sum()
}

#[aoc_runner_derive::aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    struct CardStack {
        matches: usize,
        copies:  u32,
    }

    let mut stacks: Vec<_> = input
        .lines()
        .map(|line| {
            let line = Line::<HashSet<u32>, Vec<u32>>::parse(line);
            let matches = line.right.iter().filter(|&v| line.left.contains(v)).count();
            CardStack { matches, copies: 1 }
        })
        .collect();

    let mut sum = 0;
    for i in 0..stacks.len() {
        let CardStack { matches, copies } = stacks[i];
        for j in 1..=matches {
            stacks[i + j].copies += copies;
        }
        sum += copies;
    }
    sum
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 30);
    }
}

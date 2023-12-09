use arrayvec::ArrayVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Index(u8);

fn indexer_1(b: char) -> Index {
    Index(12 - "AKQJT98765432".find(b).unwrap() as u8)
}

fn indexer_2(b: char) -> Index {
    Index(12 - "AKQT98765432J".find(b).unwrap() as u8)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    High,
    OnePair,
    TwoPairs,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

fn get_type_1(hand: [Index; 5]) -> Type {
    let mut stats = [0u8; 13];
    for card in hand {
        stats[card.0 as usize] += 1;
    }

    base_get_type(stats)
}

fn base_get_type(stats: [u8; 13]) -> Type {
    for (i, stat) in stats.into_iter().enumerate() {
        return match stat {
            5 => Type::FiveOfKind,
            4 => Type::FourOfKind,
            3 => {
                if stats[i + 1..].contains(&2) {
                    Type::FullHouse
                } else {
                    Type::ThreeOfKind
                }
            }
            2 => {
                if stats[i + 1..].contains(&3) {
                    Type::FullHouse
                } else if stats[i + 1..].contains(&2) {
                    Type::TwoPairs
                } else {
                    Type::OnePair
                }
            }
            _ => continue,
        };
    }
    Type::High
}

fn get_type_2(hand: [Index; 5]) -> Type {
    let mut stats = [0u8; 13];
    for card in hand {
        stats[card.0 as usize] += 1;
    }

    let joker = stats[0];
    let normal = &stats[1..];

    match joker {
        4 | 5 => Type::FiveOfKind,
        3 => {
            if normal.contains(&2) {
                Type::FiveOfKind
            } else {
                Type::FourOfKind
            }
        }
        2 => {
            for &stat in normal {
                match stat {
                    3 => return Type::FiveOfKind, // 2 + 3
                    2 => return Type::FourOfKind, // 2 + 2, 1
                    _ => {}                       // might have a 2 afterwards
                }
            }
            // 2 + 1, 1, 1 (still better than TwoPairs 1 + 1, 1 + 1, 1)
            Type::ThreeOfKind
        }
        1 => {
            let mut stats2 = [0u8; 5];
            for &stat in normal {
                stats2[stat as usize] += 1;
            }
            if stats2[4] == 1 {
                Type::FiveOfKind // 4 + 1
            } else if stats2[3] == 1 {
                Type::FourOfKind // 3 + 1, 1
            } else if stats2[2] == 2 {
                Type::FullHouse // 2 + 1, 2
            } else if stats2[2] == 1 {
                Type::ThreeOfKind // 2 + 1, 1, 1
            } else {
                Type::OnePair // 1 + 1, 1, 1, 1
            }
        }
        0 => base_get_type(stats),
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Line {
    ty: Type,
    hand: [Index; 5],
    bid: u16,
}

/// naive sum(a * b) algorithm
fn solve_with_mul_add(lines: &[Line]) -> u32 {
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| (i as u32 + 1) * line.bid as u32)
        .sum()
}
/// prefix sum algorithm, seems to be marginally slower due to lack of vectorization
fn solve_with_prefix_sum(lines: &[Line]) -> u32 {
    lines
        .iter()
        .rev()
        .scan(0, |state, line| {
            *state += line.bid as u32;
            Some(*state)
        })
        .sum()
}

fn solve(
    input: &str,
    indexer: impl Fn(char) -> Index + Copy,
    get_type: impl Fn([Index; 5]) -> Type,
    solver: impl Fn(&[Line]) -> u32,
) -> u32 {
    let mut lines: Vec<Line> = input
        .lines()
        .map(|line| {
            let mut line = line.chars();
            let hand: ArrayVec<Index, 5> = line.by_ref().take(5).map(indexer).collect();
            let hand = hand.into_inner().unwrap();
            let ty = get_type(hand);
            let bid = line
                .filter_map(|ch: char| ch.to_digit(10))
                .fold(0, |sum, digit| sum * 10 + digit as u16);
            Line { ty, hand, bid }
        })
        .collect();

    lines.sort_unstable();

    solver(&lines)
}

#[aoc_runner_derive::aoc(day7, part1, MulAdd)]
pub fn part1_mul_add(input: &str) -> u32 {
    solve(input, indexer_1, get_type_1, solve_with_mul_add)
}

#[aoc_runner_derive::aoc(day7, part1, PrefixSum)]
pub fn part1_prefix_sum(input: &str) -> u32 {
    solve(input, indexer_1, get_type_1, solve_with_prefix_sum)
}

#[aoc_runner_derive::aoc(day7, part2, MulAdd)]
pub fn part2_mul_add(input: &str) -> u32 {
    solve(input, indexer_2, get_type_2, solve_with_mul_add)
}

#[aoc_runner_derive::aoc(day7, part2, PrefixSum)]
pub fn part2_prefix_sum(input: &str) -> u32 {
    solve(input, indexer_2, get_type_2, solve_with_prefix_sum)
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1_prefix_sum() {
        assert_eq!(super::part1_prefix_sum(SAMPLE), 6440);
    }

    #[test]
    fn test_part2_mul_add() {
        assert_eq!(super::part2_mul_add(SAMPLE), 5905);
    }
}

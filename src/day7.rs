use arrayvec::ArrayVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Index(u8);

fn index(b: char) -> Index {
    Index(12 - "AKQJT98765432".find(b).unwrap() as u8)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Type(u8);

fn get_type(hand: [Index; 5]) -> Type {
    let mut stats = [0u8; 13];
    for card in hand {
        stats[card.0 as usize] += 1;
    }

    for (i, stat) in stats.into_iter().enumerate() {
        return Type(match stat {
            5 => 10,
            4 => 9,
            3 => {
                if stats[i + 1..].contains(&2) {
                    8
                } else {
                    7
                }
            }
            2 => {
                if stats[i + 1..].contains(&3) {
                    8
                } else if stats[i + 1..].contains(&2) {
                    6
                } else {
                    5
                }
            }
            _ => continue,
        });
    }
    Type(4)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Line {
    ty: Type,
    hand: [Index; 5],
    bid: u16,
}

#[aoc_runner_derive::aoc(day7, part1)]
pub fn part1(input: &str) -> u32 {
    let mut lines: Vec<Line> = input
        .lines()
        .map(|line| {
            let mut line = line.chars();
            let hand: ArrayVec<Index, 5> = line.by_ref().take(5).map(index).collect();
            let hand = hand.into_inner().unwrap();
            let ty = get_type(hand);
            let bid = line
                .filter_map(|ch: char| ch.to_digit(10))
                .fold(0, |sum, digit| sum * 10 + digit as u16);
            Line { ty, hand, bid }
        })
        .collect();

    lines.sort_unstable();

    lines
        .iter()
        .enumerate()
        .map(|(i, line)| (i as u32 + 1) * line.bid as u32)
        .sum()
    //
    /* // prefix sum algorithm, seems to be marginally slower due to lack of vectorization
    lines
        .iter()
        .rev()
        .scan(0, |state, line| {
            *state += line.bid as u32;
            Some(*state)
        })
        .sum()
    */
}

#[aoc_runner_derive::aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 71503);
    }
}

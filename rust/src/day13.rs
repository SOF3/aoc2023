#[aoc_runner_derive::aoc(day13, part1)]
pub fn part1(input: &str) -> u32 {
    for matrix in input.split("\n\n") {
        dbg!(matrix.lines().count(), matrix.find("\n"));
    }
    0
}

#[aoc_runner_derive::aoc(day13, part2)]
pub fn part2(input: &str) -> u32 { 0 }

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 0);
    }
}

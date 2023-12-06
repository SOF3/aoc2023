fn solve(time: u64, dist: u64) -> u64 {
    // TODO: use more efficient integer square root algorithms instead of floats
    let time = time as f64;
    let dist = dist as f64;

    // quadratic formula: +- sqrt(b^2-4ac)/2a
    let d = (time.powi(2) - 4. * dist).sqrt() / 2.;
    let left = (time / 2. - d + 1.).floor();
    let right = (time / 2. + d - 1.).ceil();
    (right - left + 1.) as u64
}

#[aoc_runner_derive::aoc(day6, part1)]
pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap());
    let dist = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap());

    time.zip(dist).map(|(t, d)| solve(t, d)).product()
}

#[aoc_runner_derive::aoc(day6, part2)]
pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .splitn(2, ':')
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let dist = lines
        .next()
        .unwrap()
        .splitn(2, ':')
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();

    solve(time, dist)
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 71503);
    }
}

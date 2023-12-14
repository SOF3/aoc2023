#[aoc_runner_derive::aoc(day14, part1)]
pub fn part1(input: &str) -> u32 {
    let width = input.find('\n').unwrap() + 1;
    let height = input.len().div_ceil(width) as u32;

    let mut gauges = vec![0u32; width - 1];
    let mut output = 0;

    for (line_no, line) in input.as_bytes().chunks(width).enumerate() {
        for (column, &ch) in line.iter().enumerate() {
            if ch == b'#' {
                gauges[column] = line_no as u32 + 1;
            } else if ch == b'O' {
                let dest = gauges[column];
                gauges[column] = dest + 1;
                output += height - dest;
            }
        }
    }

    output
}

#[aoc_runner_derive::aoc(day14, part2)]
pub fn part2(input: &str) -> u64 {
    0
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

#[aoc_runner_derive::aoc(day15, part1)]
pub fn part1(input: &str) -> u32 {
    let mut sum = 0;
    let mut hash = 0u8;
    for byte in input.bytes() {
        if byte == b',' {
            sum += hash as u32;
            hash = 0;
        } else {
            hash = hash.wrapping_add(byte).wrapping_mul(17);
        }
    }
    sum + hash as u32
}

#[aoc_runner_derive::aoc(day15, part2)]
pub fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 0);
    }
}

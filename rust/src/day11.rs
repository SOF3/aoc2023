use num_traits::{AsPrimitive, NumAssign, PrimInt};

fn add_dist<DistType: PrimInt + NumAssign>(stats: &[DistType], empty_width: DistType) -> DistType {
    // number of previous points
    let mut prev_count: DistType = DistType::zero();
    // sum of distances to each of the previous points
    let mut prev_dist: DistType = DistType::zero();
    // sum of distances between each pair completely in previous rows/columns
    let mut output: DistType = DistType::zero();

    for &stat in stats {
        output += prev_dist * stat; // add all pairs that terminate on this row/column
        prev_count += stat; // add current row/column to the set of previous points

        let my_width = if stat.is_zero() {
            empty_width
        } else {
            DistType::one()
        };
        prev_dist += prev_count * my_width; // each subsequent row/column will have more distance
                                            // the increment in prev_dist must be exactly before the next `output +=`
                                            // to prevent counting the points in the next row/column.
    }

    output
}

fn solve<DistType: PrimInt + NumAssign + 'static>(input: &[u8], empty_width: DistType) -> DistType
where
    usize: AsPrimitive<DistType>,
{
    let width = input.iter().position(|&b| b == b'\n').unwrap() + 1;

    let mut columns = vec![DistType::zero(); width - 1];
    let mut rows = vec![DistType::zero(); input.len().div_ceil(width)];

    for (y, line) in input.chunks(width).enumerate() {
        for (x, &byte) in line.iter().enumerate() {
            if byte == b'#' {
                columns[x] += DistType::one();
                rows[y] += DistType::one();
            }
        }
    }

    add_dist(&columns, empty_width) + add_dist(&rows, empty_width)
}

#[aoc_runner_derive::aoc(day11, part1)]
pub fn part1(input: &str) -> u32 {
    solve(input.as_bytes(), 2)
}

#[aoc_runner_derive::aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    solve(input.as_bytes(), 1000_000)
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::solve(SAMPLE.as_bytes(), 100), 8410);
    }
}

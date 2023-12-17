use std::collections::{hash_map, HashMap};

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

const PART2_TURNS: u32 = 1000000000;

fn tilt_vert(
    gauges: &mut Vec<u32>,
    map: &mut [u8],
    width: u32,
    line_order: impl Iterator<Item = usize>,
    offset: impl Fn(u32, u32) -> usize,
) {
    gauges.fill(0);

    for (y, line_start) in (0u32..).zip(line_order) {
        for x in 0..width {
            match &mut map[line_start + x as usize] {
                &mut b'#' => {
                    gauges[x as usize] = y + 1;
                }
                b @ &mut b'O' => {
                    let dest_y = gauges[x as usize];
                    *b = b'.';
                    map[offset(x, dest_y)] = b'O';
                    gauges[x as usize] = dest_y + 1;
                }
                _ => {}
            }
        }
    }
}

fn tilt_horiz<'t>(
    lines: impl Iterator<Item = &'t mut [u8]>,
    column_order: impl Iterator<Item = u32> + Clone,
    next_column: impl Fn(u32) -> u32,
    initial_gauge: u32,
) {
    for line in lines {
        let mut gauge = initial_gauge;
        for x in column_order.clone() {
            match &mut line[x as usize] {
                b'#' => gauge = next_column(x),
                b @ b'O' => {
                    let dest_x = gauge;
                    *b = b'.';
                    line[dest_x as usize] = b'O';
                    gauge = next_column(dest_x);
                }
                _ => {}
            }
        }
    }
}

fn compute_load<'t>(lines: impl DoubleEndedIterator<Item = &'t [u8]>) -> u32 {
    let mut output = 0;
    for (load_factor, line) in (1..).zip(lines.rev()) {
        let count = line.iter().filter(|&&b| b == b'O').count();
        output += count as u32 * load_factor;
    }
    output
}

#[aoc_runner_derive::aoc(day14, part2)]
pub fn part2(input: &str) -> u32 {
    let width = input.find('\n').unwrap() as u32;
    let full_width = width as usize + 1;
    let height = input.len().div_ceil(full_width) as u32;

    let mut map = input.as_bytes().to_vec();
    let map_len = (full_width) * height as usize;
    map.reserve_exact(map_len);
    map.resize(map_len, b'\n'); // blame cargo-aoc for this

    let mut gauges = vec![0; height as usize];
    let mut snapshot_map = HashMap::new();
    let mut snapshot_list = Vec::new();

    for turn in 0..PART2_TURNS {
        let snapshot: Box<[u8]> = Box::from(&map[..]);
        let snapshot: &'static [u8] = &Box::leak(snapshot)[..];
        snapshot_list.push(snapshot);
        match snapshot_map.entry(snapshot) {
            hash_map::Entry::Vacant(entry) => {
                entry.insert(turn);
            }
            hash_map::Entry::Occupied(entry) => {
                let &cycle_start = entry.get();
                let cycle_end = turn;

                let mut remain_turns = PART2_TURNS - cycle_end;
                remain_turns %= cycle_end - cycle_start;

                let final_map = &snapshot_list[(cycle_start + remain_turns) as usize][..];
                return compute_load(final_map.chunks_exact(full_width));
            }
        }

        tilt_vert(
            &mut gauges,
            &mut map,
            width,
            (0..map_len).step_by(width as usize + 1),
            |x, y| y as usize * full_width + x as usize,
        );
        tilt_horiz(
            map.chunks_exact_mut(full_width).map(|line| &mut line[..width as usize]),
            0..width,
            |x| x + 1,
            0,
        );
        tilt_vert(
            &mut gauges,
            &mut map,
            width,
            (0..map_len).step_by(width as usize + 1).rev(),
            |x, y| (height - 1 - y) as usize * full_width + x as usize,
        );
        tilt_horiz(
            map.chunks_exact_mut(full_width).map(|line| &mut line[..width as usize]),
            (0..width).rev(),
            |x| x.wrapping_sub(1), // x-1 may be -1 in the first column, but it doesn't matter to us
            width - 1,
        );
    }

    compute_load(map.chunks_exact(full_width))
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 64);
    }
}

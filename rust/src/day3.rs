use std::ops;

use bitvec::vec::BitVec;

struct Matrix<'t> {
    line_len: usize,
    data:     &'t [u8],
}

impl<'t> Matrix<'t> {
    fn new(data: &'t str) -> Self {
        Self { line_len: data.find('\n').unwrap() + 1, data: data.as_bytes() }
    }

    fn has_x(&self, x: i16) -> bool { (0..self.line_len as i16 - 1).contains(&x) }

    fn has_y(&self, y: i16) -> bool { (0..=(self.data.len() / self.line_len) as i16).contains(&y) }

    fn to_index(&self, coord: Vec2) -> Option<usize> {
        if self.has_x(coord.1) && self.has_y(coord.1) {
            Some(coord.0 as usize + self.line_len * coord.1 as usize)
        } else {
            None
        }
    }

    fn to_coord(&self, index: usize) -> Vec2 {
        Vec2((index % self.line_len) as i16, (index / self.line_len) as i16)
    }
}

#[derive(Clone, Copy)]
struct Vec2(i16, i16);

struct IterNumbers<'t> {
    scan:   &'t str,
    offset: usize,
}

impl<'t> Iterator for IterNumbers<'t> {
    type Item = (ops::Range<usize>, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(noise_len) = self.scan.find(|ch: char| ch.is_ascii_digit()) else {
            return None;
        };

        self.scan = &self.scan[noise_len..];
        self.offset += noise_len;
        let num_start = self.offset;

        let num_len = self.scan.find(|ch: char| !ch.is_ascii_digit()).unwrap_or(self.scan.len());
        let parsed = self.scan[..num_len].parse::<u32>().unwrap();

        self.scan = &self.scan[num_len..];
        self.offset += num_len;

        Some((num_start..self.offset, parsed))
    }
}

fn iter_numbers(scan: &str) -> impl Iterator<Item = (ops::Range<usize>, u32)> + '_ {
    IterNumbers { scan, offset: 0 }
}

#[aoc_runner_derive::aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let mat = Matrix::new(input);
    let mut active: BitVec<usize, bitvec::order::LocalBits> = BitVec::repeat(false, input.len());

    input.match_indices(|ch: char| !ch.is_ascii_digit() && ch != '.' && ch != '\n').for_each(
        |(index, _)| {
            let vec = mat.to_coord(index);
            let (mut x_start, mut x_end) = (vec.0 - 1, vec.0 + 1);
            if !mat.has_x(x_start) {
                x_start += 1;
            }
            if !mat.has_x(x_end) {
                x_end -= 1;
            }
            for y in (vec.1 - 1)..=(vec.1 + 1) {
                if mat.has_y(y) {
                    let start_index = mat.to_index(Vec2(x_start, y)).unwrap();
                    let end_index = start_index + (x_end - x_start) as usize;
                    active[start_index..=end_index].fill(true);
                }
            }
        },
    );

    iter_numbers(input).filter(|(range, _)| active[range.clone()].any()).map(|(_, num)| num).sum()
}

fn lookup_number(suffix: &str) -> u32 {
    suffix[..suffix.find(|ch: char| !ch.is_ascii_digit()).unwrap_or(suffix.len())]
        .parse::<u32>()
        .unwrap()
}

fn reverse_lookup_number(prefix: &str) -> u32 {
    let start = match prefix.rfind(|ch: char| !ch.is_ascii_digit()) {
        Some(sym) => sym + 1,
        None => 0,
    };
    prefix[start..].parse::<u32>().unwrap()
}

fn lookup_number_around(buf: &str, offset: usize) -> u32 {
    let start = match buf[..offset].rfind(|ch: char| !ch.is_ascii_digit()) {
        Some(sym) => sym + 1,
        None => 0,
    };
    let end = match buf[offset..].find(|ch: char| !ch.is_ascii_digit()) {
        Some(sym) => offset + sym,
        None => buf.len(),
    };
    buf[start..end].parse::<u32>().unwrap()
}

#[aoc_runner_derive::aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let mat = Matrix::new(input);

    let mut adjs = Vec::with_capacity(2);
    let mut sum = 0;

    'next_gear: for (gear_index, _) in input.match_indices('*') {
        let gear_coord = mat.to_coord(gear_index);
        adjs.clear();

        let seek_disjoint = |y: i16, adjs: &mut Vec<u32>| {
            if let Some(left) = mat.to_index(Vec2(gear_coord.0 - 1, y)) {
                if (mat.data[left] as char).is_ascii_digit() {
                    if adjs.len() >= 2 {
                        return true;
                    }
                    adjs.push(reverse_lookup_number(&input[..=left]));
                }
            }

            if let Some(right) = mat.to_index(Vec2(gear_coord.0 + 1, y)) {
                if (mat.data[right] as char).is_ascii_digit() {
                    if adjs.len() >= 2 {
                        return true;
                    }
                    adjs.push(lookup_number(&input[right..]));
                }
            }

            false
        };

        seek_disjoint(gear_coord.1, &mut adjs);

        for y in [gear_coord.1 - 1, gear_coord.1 + 1] {
            if let Some(mid) = mat.to_index(Vec2(gear_coord.0, y)) {
                if (mat.data[mid] as char).is_ascii_digit() {
                    if adjs.len() >= 2 {
                        continue 'next_gear;
                    }
                    adjs.push(lookup_number_around(input, mid))
                } else {
                    let overflow = seek_disjoint(y, &mut adjs);
                    if overflow {
                        continue 'next_gear;
                    }
                }
            }
        }

        if adjs.len() == 2 {
            sum += adjs[0] * adjs[1];
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 467835);
    }
}

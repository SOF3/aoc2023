use std::iter;

use bitvec::vec::BitVec;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ]
    }

    fn follow(self, from: usize, width: usize, full_len: usize) -> Option<usize> {
        match self {
            Self::Left if from % width != 0 => Some(from - 1),
            Self::Right if from % width != width - 2 => Some(from + 1),
            Self::Up => from.checked_sub(width),
            Self::Down if from + width < full_len => Some(from + width),
            _ => None,
        }
    }

    fn reflect(self, ch: u8) -> ReflectResult {
        match (self, ch) {
            (_, b'.') => self.into(),
            (Self::Left | Self::Right, b'-') | (Self::Up | Self::Down, b'|') => self.into(),
            (Self::Up | Self::Down, b'-') => [Self::Left, Self::Right].into(),
            (Self::Left | Self::Right, b'|') => [Self::Up, Self::Down].into(),
            (Self::Right, b'/') => Self::Up.into(),
            (Self::Down, b'/') => Self::Left.into(),
            (Self::Left, b'/') => Self::Down.into(),
            (Self::Up, b'/') => Self::Right.into(),
            (Self::Right, b'\\') => Self::Down.into(),
            (Self::Down, b'\\') => Self::Right.into(),
            (Self::Left, b'\\') => Self::Up.into(),
            (Self::Up, b'\\') => Self::Left.into(),
            _ => unreachable!("{self:?} on {ch}"),
        }
    }

    fn neg(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

struct ReflectResult(Direction, Option<Direction>);
impl From<Direction> for ReflectResult {
    fn from(value: Direction) -> Self {
        Self(value, None)
    }
}
impl From<[Direction; 2]> for ReflectResult {
    fn from([v1, v2]: [Direction; 2]) -> Self {
        Self(v1, Some(v2))
    }
}
impl IntoIterator for ReflectResult {
    type Item = Direction;
    type IntoIter = impl Iterator<Item = Direction>;
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self.0).chain(self.1)
    }
}

fn trace(energized: &mut [BitVec; 4], input: &[u8], width: usize, pos: usize, dir: Direction) {
    if energized[dir as usize].replace(pos, true) {
        return; // already traced
    }
    for new_dir in dir.reflect(input[pos]) {
        if let Some(new_pos) = new_dir.follow(pos, width, input.len()) {
            trace(energized, input, width, new_pos, new_dir);
        }
    }
}

fn solve(input: &[u8], width: usize, initial_pos: usize, initial_dir: Direction) -> [BitVec; 4] {
    let mut energized = std::array::from_fn(|_| BitVec::repeat(false, input.len()));
    trace(&mut energized, input, width, initial_pos, initial_dir);
    energized
}

#[aoc_runner_derive::aoc(day16, part1)]
pub fn part1(input: &str) -> u32 {
    let width = input.find('\n').unwrap() + 1;
    let input = input.as_bytes();

    solve(input, width, 0, Direction::Right)
        .into_iter()
        .reduce(|a, b| a | b)
        .unwrap()
        .count_ones() as u32
}

#[aoc_runner_derive::aoc(day16, part2)]
pub fn part2(input: &str) -> u32 {
    let width = input.find('\n').unwrap() + 1;
    let height = input.len().div_ceil(width);
    let input = input.as_bytes();

    let mut historical: [BitVec; 4] = std::array::from_fn(|_| BitVec::repeat(false, input.len()));

    let mut max_energy = 0;

    for (initial_pos, initial_dir) in (0..input.len())
        .step_by(width)
        .flat_map(|head| {
            [
                (head, Direction::Right),
                (head + width - 2, Direction::Left),
            ]
        })
        .chain((0..width - 1).flat_map(|x| {
            [
                (x, Direction::Down),
                (x + width * (height - 1), Direction::Up),
            ]
        }))
    {
        if Direction::all().into_iter().any(|incident_dir| {
            historical[incident_dir as usize][initial_pos]
                && incident_dir
                    .reflect(input[initial_pos])
                    .into_iter()
                    .any(|new_dir| new_dir == initial_dir.neg())
        }) {
            continue;
        }

        let energized = solve(input, width, initial_pos, initial_dir);
        for (hist, new) in historical.iter_mut().zip(energized.iter()) {
            *hist |= new;
        }

        let energy = energized
            .into_iter()
            .reduce(|a, b| a | b)
            .unwrap()
            .count_ones() as u32;
        max_energy = max_energy.max(energy);
    }

    max_energy
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 51);
    }
}

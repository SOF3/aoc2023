use std::fmt::Display;

use bitvec::vec::BitVec;

type PosType = u32;

struct Grid<'t> {
    buf:            &'t [u8],
    width_plus_one: PosType,
}

impl<'t> Grid<'t> {
    fn parse(buf: &'t str) -> Self {
        let width_plus_one = (buf.find('\n').unwrap() + 1) as PosType;
        Self { buf: buf.as_bytes(), width_plus_one }
    }

    fn next_dir(&self, current_pos: Pos, source_dir: Dir) -> Option<Dir> {
        source_dir.follow_char(self.buf[current_pos.into_usize()])
    }

    fn print(&self, pos: Pos) -> impl Display {
        let x = pos.0 % self.width_plus_one;
        let y = pos.0 / self.width_plus_one;
        let ch = self.buf[pos.into_usize()] as char;
        format!("({x}, {y}) ({ch:?})")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pos(PosType);

impl From<usize> for Pos {
    fn from(value: usize) -> Self { Self(value as PosType) }
}

impl Pos {
    fn into_usize(self) -> usize { self.0 as usize }

    fn up(&self, grid: &Grid<'_>) -> Option<Self> {
        self.0.checked_sub(grid.width_plus_one).map(Self)
    }
    fn down(&self, grid: &Grid<'_>) -> Option<Self> {
        let new = self.0 + grid.width_plus_one;
        if new as usize >= grid.buf.len() {
            None
        } else {
            Some(Self(new))
        }
    }
    fn left(&self, grid: &Grid<'_>) -> Option<Self> {
        if self.0 % grid.width_plus_one == 0 {
            None
        } else {
            Some(Self(self.0 - 1))
        }
    }
    fn right(&self, grid: &Grid<'_>) -> Option<Self> {
        if self.0 % grid.width_plus_one == grid.width_plus_one - 2 {
            None
        } else {
            Some(Self(self.0 + 1))
        }
    }

    fn go(&self, grid: &Grid<'_>, dir: Dir) -> Option<Self> {
        match dir {
            Dir::Up => self.up(grid),
            Dir::Down => self.down(grid),
            Dir::Left => self.left(grid),
            Dir::Right => self.right(grid),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn follow_char(self, ch: u8) -> Option<Dir> {
        Some(match self {
            Self::Up => match ch {
                b'|' => Self::Down,
                b'J' => Self::Left,
                b'L' => Self::Right,
                _ => return None,
            },
            Self::Down => match ch {
                b'|' => Self::Up,
                b'7' => Self::Left,
                b'F' => Self::Right,
                _ => return None,
            },
            Self::Left => match ch {
                b'-' => Self::Right,
                b'J' => Self::Up,
                b'7' => Self::Down,
                _ => return None,
            },
            Self::Right => match ch {
                b'-' => Self::Left,
                b'L' => Self::Up,
                b'F' => Self::Down,
                _ => return None,
            },
        })
    }

    fn neg(self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

fn identify_loop(input: &str, mut step: impl FnMut(Dir, Pos, Dir)) {
    let grid = Grid::parse(input);
    let initial = Pos::from(input.find('S').unwrap());

    let mut pos = initial;
    let initial_dir = [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
        .into_iter()
        .filter(|&dir| match initial.go(&grid, dir) {
            Some(next) => dir.neg().follow_char(grid.buf[next.into_usize()]).is_some(),
            None => false,
        })
        .next()
        .unwrap();

    let mut next_dir = initial_dir;

    loop {
        pos = pos.go(&grid, next_dir).unwrap();
        // eprintln!("go to {} after heading {next_dir:?}", grid.print(pos));
        step(initial_dir, pos, next_dir);
        if pos == initial {
            break;
        }
        next_dir = match grid.next_dir(pos, next_dir.neg()) {
            Some(dir) => dir,
            None => {
                panic!("at {}: cannot follow {} after heading {next_dir:?}", pos.0, grid.print(pos))
            }
        };
    }
}

#[aoc_runner_derive::aoc(day10, part1)]
pub fn part1(input: &str) -> PosType {
    let mut count = 0;
    identify_loop(input, |_, _, _| count += 1);
    count / 2
}

trait Marker: Sized {
    fn init(len: usize) -> Self;
    fn mark(&mut self, pos: usize, up: bool, down: bool);
    fn flush(&mut self) {}
    fn iter_halves(&self) -> impl Iterator<Item = (bool, PosType)> + '_;
}

#[aoc_runner_derive::aoc(day10, part2, BitVec)]
pub fn part2_bitvec(input: &str) -> PosType { part2::<BitVec>(input) }

impl Marker for BitVec {
    fn init(len: usize) -> Self { Self::repeat(false, len * 2) }

    fn mark(&mut self, pos: usize, up: bool, down: bool) {
        if up {
            self.set(pos * 2, true)
        }
        if down {
            self.set(pos * 2 + 1, true)
        }
    }

    fn iter_halves(&self) -> impl Iterator<Item = (bool, PosType)> + '_ {
        self.iter_ones().map(|pos| (pos % 2 == 0, (pos / 2) as PosType))
    }
}

#[aoc_runner_derive::aoc(day10, part2, ByteVec)]
pub fn part2_bytevec(input: &str) -> PosType { part2::<Vec<u8>>(input) }

impl Marker for Vec<u8> {
    fn init(len: usize) -> Self { vec![0u8; len] }

    fn mark(&mut self, pos: usize, up: bool, down: bool) {
        if up {
            self[pos] |= 1
        }
        if down {
            self[pos] |= 2
        }
    }

    fn iter_halves(&self) -> impl Iterator<Item = (bool, PosType)> + '_ {
        self.iter().copied().enumerate().flat_map(|(pos, mark)| {
            let up = (mark & 1 > 0).then_some((true, pos as PosType));
            let down = (mark & 2 > 0).then_some((false, pos as PosType));
            [up, down].into_iter().flatten()
        })
    }
}

#[aoc_runner_derive::aoc(day10, part2, MarkList)]
pub fn part2_marklist(input: &str) -> PosType { part2::<Vec<(PosType, u8)>>(input) }

impl Marker for Vec<(PosType, u8)> {
    fn init(len: usize) -> Self { Vec::with_capacity(len) }

    fn mark(&mut self, pos: usize, up: bool, down: bool) {
        let mut mark = 0u8;
        if up {
            mark |= 1
        }
        if down {
            mark |= 2
        }
        self.push((pos as PosType, mark));
    }

    fn flush(&mut self) { self.sort_by_key(|(pos, _)| *pos); }

    fn iter_halves(&self) -> impl Iterator<Item = (bool, PosType)> + '_ {
        self.iter().flat_map(|&(pos, mark)| {
            let up = (mark & 1 > 0).then_some((true, pos));
            let down = (mark & 2 > 0).then_some((false, pos));
            [up, down].into_iter().flatten()
        })
    }
}

fn part2<V: Marker>(input: &str) -> PosType {
    let mut bv = V::init(input.len());
    identify_loop(input, |initial_dir, pos, last_dir| match input.as_bytes()[pos.into_usize()] {
        b'J' | b'L' => bv.mark(pos.into_usize(), true, false),
        b'7' | b'F' => bv.mark(pos.into_usize(), false, true),
        b'|' => {
            bv.mark(pos.into_usize(), true, true);
        }
        b'S' => bv.mark(
            pos.into_usize(),
            initial_dir == Dir::Up || last_dir.neg() == Dir::Up,
            initial_dir == Dir::Down || last_dir.neg() == Dir::Down,
        ),
        _ => {}
    });
    bv.flush();

    let mut up_set = false;
    let mut down_set = false;
    let mut last_pos = 0;
    let mut output: PosType = 0;
    for (is_up, pos) in bv.iter_halves() {
        if up_set && down_set {
            output += pos - last_pos - 1;
        }
        if is_up {
            up_set = !up_set
        } else {
            down_set = !down_set
        }
        last_pos = pos;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::PosType;

    const SAMPLE1: &str = r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE1), 8);
    }

    const SAMPLE2: &str = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    macro_rules! test_part2 {
        ($name:ident, $marker:ty) => {
            paste::paste! {
                #[test]
                fn [<test_part2_ $name _sample1>]() {
                    assert_eq!(super::part2::<$marker>(SAMPLE1), 1);
                }

                #[test]
                fn [<test_part2_ $name _sample2>]() {
                    assert_eq!(super::part2::<$marker>(SAMPLE2), 10);
                }
            }
        };
    }

    test_part2!(bitvec, bitvec::vec::BitVec);
    test_part2!(bytevec, Vec<u8>);
    test_part2!(marklist, Vec<(PosType, u8)>);
}

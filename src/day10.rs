use std::fmt::Display;

struct Grid<'t> {
    buf: &'t [u8],
    width_plus_one: usize,
    height: usize,
}

impl<'t> Grid<'t> {
    fn parse(buf: &'t str) -> Self {
        let width_plus_one = buf.find('\n').unwrap() + 1;
        Self {
            buf: buf.as_bytes(),
            width_plus_one,
            height: buf.len() / width_plus_one,
        }
    }

    fn next_dir(&self, current_pos: Pos, source_dir: Dir) -> Option<Dir> {
        source_dir.follow_char(self.buf[current_pos.0])
    }

    fn print(&self, pos: Pos) -> impl Display {
        let x = pos.0 % self.width_plus_one;
        let y = pos.0 / self.width_plus_one;
        let ch = self.buf[pos.0] as char;
        format!("({x}, {y}) ({ch:?})")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pos(usize);

impl Pos {
    fn up(&self, grid: &Grid<'_>) -> Option<Self> {
        self.0.checked_sub(grid.width_plus_one).map(Self)
    }
    fn down(&self, grid: &Grid<'_>) -> Option<Self> {
        let new = self.0 + grid.width_plus_one;
        if new >= grid.buf.len() {
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

#[derive(Debug, Clone, Copy)]
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

#[aoc_runner_derive::aoc(day10, part1)]
pub fn part1(input: &str) -> u32 {
    let grid = Grid::parse(input);
    let initial = Pos(input.find('S').unwrap());

    let mut pos = initial;
    let mut next_dir = [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
        .into_iter()
        .filter(|&dir| match initial.go(&grid, dir) {
            Some(next) => dir.neg().follow_char(grid.buf[next.0]).is_some(),
            None => false,
        })
        .next()
        .unwrap();

    let mut count = 0;
    loop {
        pos = pos.go(&grid, next_dir).unwrap();
        // eprintln!("go to {} after heading {next_dir:?}", grid.print(pos));
        count += 1;
        if pos == initial {
            break count / 2;
        }
        next_dir = match grid.next_dir(pos, next_dir.neg()) {
            Some(dir) => dir,
            None => panic!(
                "step {count} at {}: cannot follow {} after heading {next_dir:?}",
                pos.0,
                grid.print(pos)
            ),
        };
    }
}

#[aoc_runner_derive::aoc(day10, part2)]
pub fn part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 0);
    }
}

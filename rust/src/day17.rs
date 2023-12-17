use std::collections::BinaryHeap;
use std::{array, cmp, fmt};

macro_rules! log {
    ($($tt:tt)*) => {
        // println!($($tt)*)
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn all() -> [Dir; 4] { [Self::Up, Self::Down, Self::Left, Self::Right] }

    fn neg(self) -> Dir {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn display(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.display()) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos(u32);

struct Grid<'t> {
    input: &'t [u8],
    width: u32,
}

impl<'t> Grid<'t> {
    fn go(&self, pos: Pos, dir: Dir) -> Option<Pos> {
        Some(Pos(match dir {
            Dir::Up => pos.0.checked_sub(self.width)?,
            Dir::Down if pos.0 + self.width < self.input.len() as u32 => pos.0 + self.width,
            Dir::Left if pos.0 % self.width != 0 => pos.0 - 1,
            Dir::Right if pos.0 % self.width != self.width - 2 => pos.0 + 1,
            _ => return None,
        }))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Path {
    /// accumulative cost including current position
    total_cost:  cmp::Reverse<u32>,
    /// number of previous steps in last_dir including step to current position
    /// always 1 to 3 except 0 during init
    dir_steps:   u32,
    /// direction entering current position
    last_dir:    Dir,
    /// current position
    pos:         Pos,
    debug_trace: DebugTrace,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct DebugTrace(Vec<(Pos, Dir, u32)>);

impl DebugTrace {
    fn observe(&self, pos: Pos, dir: Dir, cost: u32) -> Self {
        let mut new = self.clone();
        new.0.push((pos, dir, cost));
        new
    }

    fn print(&self, grid: &[u8]) -> String {
        let mut grid = grid.to_vec();
        for (pos, dir, _) in &self.0 {
            grid[pos.0 as usize] = dir.display() as u8;
        }
        String::from_utf8(grid).unwrap()
    }
}

fn cost_of(b: u8) -> u32 { (b - b'0') as u32 }

fn trace(grid: Grid<'_>) -> u32 {
    let mut heap = BinaryHeap::new();
    heap.push(Path {
        total_cost:  cmp::Reverse(0),
        dir_steps:   0,
        last_dir:    Dir::Down, // arbitrary, as long as it is not opposite to any of the desired
        pos:         Pos(0),
        debug_trace: DebugTrace::default(),
    });

    let mut visited_dirs: [_; 4] = array::from_fn(|_| vec![0; grid.input.len()]);

    loop {
        let path = heap.pop().unwrap();

        log!("cost: {}, path:", path.total_cost.0);
        log!("{}", path.debug_trace.print(grid.input));

        if path.pos.0 == grid.input.len() as u32 - 1 {
            log!("{:?}", &path.debug_trace);
            break path.total_cost.0;
        }

        for next_dir in Dir::all() {
            if next_dir == path.last_dir.neg() {
                log!("skip neg {next_dir}");
                continue;
            } // no reverse
            let Some(next_pos) = grid.go(path.pos, next_dir) else {
                log!("skip wall {next_dir}");
                continue;
            }; // wall
            let cost_inc = cost_of(grid.input[next_pos.0 as usize]);
            let next_cost = path.total_cost.0 + cost_inc;
            let next_dir_steps = if next_dir == path.last_dir { path.dir_steps + 1 } else { 1 };
            if next_dir_steps > 3 {
                log!("skip straight {next_dir}");
                continue;
            }

            let visited_dir = &mut visited_dirs[next_dir as usize];
            let visited = &mut visited_dir[next_pos.0 as usize];
            if *visited >= 4 - next_dir_steps {
                log!("skip visited {next_dir}");
                continue;
            }
            *visited = 4 - next_dir_steps;

            let next_path = Path {
                total_cost:  cmp::Reverse(next_cost),
                dir_steps:   next_dir_steps,
                last_dir:    next_dir,
                pos:         next_pos,
                debug_trace: path.debug_trace.observe(next_pos, next_dir, cost_inc),
            };
            heap.push(next_path);
            log!("push {next_dir}");
        }
    }
}

#[aoc_runner_derive::aoc(day17, part1)]
pub fn part1(input: &str) -> u32 {
    let width = input.find('\n').unwrap() + 1;

    trace(Grid { input: input.trim_end_matches('\n').as_bytes(), width: width as u32 })
}

#[aoc_runner_derive::aoc(day17, part2)]
pub fn part2(input: &str) -> u32 { 0 }

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 0);
    }
}

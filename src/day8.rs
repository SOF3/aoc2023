use bitvec::vec::BitVec;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
struct NodeId(u16);

impl NodeId {
    const REQUIRED_SPACE: usize = 26 * 26 * 26;

    const AAA: Self = Self::new([b'A', b'A', b'A']);
    const AAZ: Self = Self::new([b'A', b'A', b'Z']);
    const ZZZ: Self = Self::new([b'Z', b'Z', b'Z']);

    const fn new(arr: [u8; 3]) -> Self {
        let d0 = (arr[0] - b'A') as u16;
        let d1 = (arr[1] - b'A') as u16;
        let d2 = (arr[2] - b'A') as u16;
        Self(d2 * 26 * 26 + d1 * 26 + d0)
    }

    fn usize(self) -> usize {
        self.0 as usize
    }

    fn from_usize(i: usize) -> Self {
        Self(i as u16)
    }
}

#[derive(Clone, Copy, Default)]
struct Node {
    left: NodeId,
    right: NodeId,
}

impl Node {
    fn parse(line: &str) -> (NodeId, Node) {
        let line = line.as_bytes();
        let src = NodeId::new(line[0..3].try_into().unwrap());
        let left = NodeId::new(line[7..10].try_into().unwrap());
        let right = NodeId::new(line[12..15].try_into().unwrap());
        (src, Node { left, right })
    }

    fn go(self, dir: Dir) -> NodeId {
        match dir {
            Dir::Left => self.left,
            Dir::Right => self.right,
        }
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Left,
    Right,
}

fn parse_steps(line: &str) -> impl Iterator<Item = Dir> + Clone + '_ {
    line.chars()
        .map(|c| match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => unsafe{std::hint::unreachable_unchecked()},
        })
        .cycle()
}

fn parse_graph<'t>(
    lines: impl Iterator<Item = &'t str>,
    mut mark: impl FnMut(NodeId),
) -> Graph {
    let mut nodes = [Node::default(); NodeId::REQUIRED_SPACE];
    for line in lines {
        if !line.is_empty() {
            let (src, node) = Node::parse(line);
            nodes[src.usize()] = node;
            mark(src);
        }
    }

    Graph{nodes}
}

struct Graph {
    nodes: [Node; NodeId::REQUIRED_SPACE],
}

impl Graph {
    fn count(&self, initial: NodeId, stepper: impl Iterator<Item = Dir>, terminate: impl Fn(NodeId) -> bool) -> u32 {
        let mut state = initial;
        for (steps, dir) in (0..).zip(stepper) {
            if terminate(state) {
                return steps;
            }
            state = self.nodes[state.usize()].go(dir);
        }

        unreachable!()
    }
}

#[aoc_runner_derive::aoc(day8, part1)]
pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let stepper = parse_steps(lines.next().unwrap());
    let graph = parse_graph(lines, |_| {});

    graph.count(NodeId::AAA, stepper, |node| node == NodeId::ZZZ)
}

#[aoc_runner_derive::aoc(day8, part2, EmpiricalProd)]
pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut present: BitVec = BitVec::repeat(false, NodeId::REQUIRED_SPACE);

    let steps_line = lines.next().unwrap();
    let stepper = parse_steps(steps_line);
    let graph = parse_graph(lines, |node| present.set(node.usize(), true));

    present[..676].iter_ones().map(|one| {
        let count = graph.count(NodeId::from_usize(one), stepper.clone(), |node| NodeId::AAZ <= node && node <= NodeId::ZZZ);
        (count as u64 / steps_line.len() as u64) as u64
    }).product::<u64>() * steps_line.len() as u64
}

#[cfg(test)]
mod tests {
    const SAMPLE1: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE1), 6);
    }
}

use bitvec::vec::BitVec;

trait NodeId: Clone + Copy + Default + Eq + Ord {
    fn new(arr: [u8; 3]) -> Self;
    fn u16(self) -> u16;
    fn from_u16(i: u16) -> Self;
    fn usize(self) -> usize;
    fn from_usize(i: usize) -> Self;
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Packed(u16);

impl NodeId for Packed {
    fn new(arr: [u8; 3]) -> Self {
        let d0 = (arr[0] - b'A') as u16;
        let d1 = (arr[1] - b'A') as u16;
        let d2 = (arr[2] - b'A') as u16;
        Self(d2 * 26 * 26 + d1 * 26 + d0)
    }

    fn u16(self) -> u16 { self.0 }
    fn from_u16(i: u16) -> Self { Self(i) }
    fn usize(self) -> usize { self.0 as usize }
    fn from_usize(i: usize) -> Self { Self(i as u16) }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
struct BitShift(u16);

impl NodeId for BitShift {
    fn new(arr: [u8; 3]) -> Self {
        let d0 = (arr[0] - b'A') as u16;
        let d1 = (arr[1] - b'A') as u16;
        let d2 = (arr[2] - b'A') as u16;
        Self(d2 << 10 | d1 << 5 | d0)
    }

    fn u16(self) -> u16 { self.0 }
    fn from_u16(i: u16) -> Self { Self(i) }
    fn usize(self) -> usize { self.0 as usize }
    fn from_usize(i: usize) -> Self { Self(i as u16) }
}

#[derive(Clone, Copy, Default)]
struct Node<NodeIdT: NodeId> {
    left:  NodeIdT,
    right: NodeIdT,
}

impl<NodeIdT: NodeId> Node<NodeIdT> {
    fn parse(line: &str) -> (NodeIdT, Node<NodeIdT>) {
        let line = line.as_bytes();
        let src = NodeIdT::new(line[0..3].try_into().unwrap());
        let left = NodeIdT::new(line[7..10].try_into().unwrap());
        let right = NodeIdT::new(line[12..15].try_into().unwrap());
        (src, Node { left, right })
    }

    fn go(self, dir: Dir) -> NodeIdT {
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
            _ => unreachable!(),
        })
        .cycle()
}

fn parse_graph<'t, NodeIdT: NodeId>(
    lines: impl Iterator<Item = &'t str>,
    mut mark: impl FnMut(NodeIdT),
) -> Graph<NodeIdT> {
    let mut nodes = [Node::default(); u16::MAX as usize];
    for line in lines {
        if !line.is_empty() {
            let (src, node) = Node::<NodeIdT>::parse(line);
            nodes[src.usize()] = node;
            mark(src);
        }
    }

    Graph { nodes }
}

struct Graph<NodeIdT: NodeId> {
    nodes: [Node<NodeIdT>; u16::MAX as usize],
}

impl<NodeIdT: NodeId> Graph<NodeIdT> {
    fn count(
        &self,
        initial: NodeIdT,
        stepper: impl Iterator<Item = Dir>,
        terminate: impl Fn(NodeIdT) -> bool,
    ) -> u32 {
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

fn aaa<NodeIdT: NodeId>() -> NodeIdT { NodeIdT::new(*b"AAA") }
fn aaz<NodeIdT: NodeId>() -> NodeIdT { NodeIdT::new(*b"AAZ") }
fn zzz<NodeIdT: NodeId>() -> NodeIdT { NodeIdT::new(*b"ZZZ") }

#[aoc_runner_derive::aoc(day8, part1, Packed)]
pub fn part1_packed(input: &str) -> u32 { part1::<Packed>(input) }
#[aoc_runner_derive::aoc(day8, part1, BitShift)]
pub fn part1_bitshift(input: &str) -> u32 { part1::<BitShift>(input) }

fn part1<NodeIdT: NodeId>(input: &str) -> u32 {
    let mut lines = input.lines();
    let stepper = parse_steps(lines.next().unwrap());
    let graph: Graph<NodeIdT> = parse_graph(lines, |_| {});

    graph.count(aaa(), stepper, |node| node == zzz())
}

#[aoc_runner_derive::aoc(day8, part2, EmpiricalProd_Packed)]
pub fn part2_packed(input: &str) -> u64 { part2::<Packed>(input) }
#[aoc_runner_derive::aoc(day8, part2, EmpiricalProd_BitShift)]
pub fn part2_bitshift(input: &str) -> u64 { part2::<BitShift>(input) }

fn part2<NodeIdT: NodeId>(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut present: BitVec = BitVec::repeat(false, u16::MAX as usize);

    let steps_line = lines.next().unwrap();
    let stepper = parse_steps(steps_line);
    let graph: Graph<NodeIdT> = parse_graph(lines, |node: NodeIdT| present.set(node.usize(), true));

    present[..676]
        .iter_ones()
        .map(|one| {
            let count = graph.count(NodeIdT::from_usize(one), stepper.clone(), |node| {
                node >= aaz() && node <= zzz()
            });
            (count as u64 / steps_line.len() as u64) as u64
        })
        .product::<u64>()
        * steps_line.len() as u64
}

#[cfg(test)]
mod tests {
    use crate::day8::{BitShift, Packed};

    const SAMPLE1: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part1_packed() {
        assert_eq!(super::part1::<Packed>(SAMPLE1), 6);
    }
    #[test]
    fn test_part1_bitshift() {
        assert_eq!(super::part1::<BitShift>(SAMPLE1), 6);
    }
}

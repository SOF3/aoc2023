#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct NodeId(u16);

impl NodeId {
    const AAA: Self = Self::new([b'A', b'A', b'A']);
    const ZZZ: Self = Self::new([b'Z', b'Z', b'Z']);

    const fn new(arr: [u8; 3]) -> Self {
        let d0 = (arr[0] - b'A') as u16;
        let d1 = (arr[1] - b'A') as u16;
        let d2 = (arr[2] - b'A') as u16;
        Self(d0 * 26 * 26 + d1 * 26 + d2)
    }

    fn usize(self) -> usize {
        self.0 as usize
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

#[aoc_runner_derive::aoc(day8, part1)]
pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let steps = lines.next().unwrap();
    let stepper = steps
        .chars()
        .map(|c| match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => unreachable!(),
        })
        .cycle();

    let mut nodes = vec![Node::default(); 26 * 26 * 26];
    for line in lines {
        if !line.is_empty() {
            let (src, node) = Node::parse(line);
            nodes[src.usize()] = node;
        }
    }

    let mut loc = NodeId::AAA;
    for (steps, dir) in (0..).zip(stepper) {
        if loc == NodeId::ZZZ {
            return steps;
        }
        loc = nodes[loc.usize()].go(dir);
    }

    unreachable!()
}

#[aoc_runner_derive::aoc(day8, part2)]
pub fn part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 71503);
    }
}

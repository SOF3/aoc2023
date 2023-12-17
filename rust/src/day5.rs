use std::iter;

#[derive(Default)]
struct Map {
    sort_src: Vec<Triple>,
}

impl Map {
    fn find_src_sorted(&self, src: u64) -> u64 {
        let Some(triple) = self.sort_src[..self.sort_src.partition_point(|t| t.src <= src)].last()
        else {
            return src;
        };
        let delta = src - triple.src;
        if delta < triple.width {
            triple.dest + delta
        } else {
            src
        }
    }

    fn find_src_range_sorted(&self, src: Range) -> impl Iterator<Item = Range> + '_ {
        let left_pp = self.sort_src.partition_point(|triple| triple.src + triple.width < src.start);
        let mut relevant = &self.sort_src[left_pp..];
        relevant =
            &relevant[..relevant.partition_point(|triple| triple.src < src.start + src.width)];

        let mut last_end = src.start;
        let final_end = src.start + src.width;

        let last_relevant = relevant.last().copied();

        relevant
            .iter()
            .flat_map(move |triple| {
                // region without mapping
                let gap = Range { start: last_end, width: triple.src.saturating_sub(last_end) };
                last_end += gap.width;
                // region with mapping
                let mapped = Range {
                    start: triple.dest + (last_end - triple.src),
                    width: (triple.src + triple.width).min(final_end) - last_end,
                };
                last_end += mapped.width;
                [gap, mapped]
            })
            .chain(iter::once_with(move || {
                match last_relevant {
                    Some(last) => {
                        // gap from last relevant
                        Range {
                            start: last.src + last.width,
                            width: final_end.saturating_sub(last.src + last.width),
                        }
                    }
                    None => {
                        // relevant is empty, i.e. no overlapping triples
                        src
                    }
                }
            }))
            .filter(|range| range.width > 0)
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    width: u64,
}

#[derive(Debug, Clone, Copy)]
struct Triple {
    dest:  u64,
    src:   u64,
    width: u64,
}

fn parse_input(input: &str) -> (impl Iterator<Item = u64> + '_, Vec<Map>) {
    let (seeds, input) = input.split_once('\n').unwrap();
    let seeds =
        seeds.strip_prefix("seeds: ").unwrap().split(' ').map(|s| s.parse::<u64>().unwrap());

    let mut maps = Vec::new();
    let mut active_map = None;

    for line in input.lines().filter(|line| !line.is_empty()) {
        if let Some(_title) = line.strip_suffix(" map:") {
            maps.push(Map::default());
            active_map = maps.last_mut();
            continue;
        }

        let mut iter = line.splitn(3, ' ').map(|s| match s.parse::<u64>() {
            Ok(x) => x,
            Err(err) => panic!("parse {s:?}: {err}"),
        });
        let triple = Triple {
            dest:  iter.next().unwrap(),
            src:   iter.next().unwrap(),
            width: iter.next().unwrap(),
        };

        let map = active_map.as_mut().unwrap();
        map.sort_src.push(triple);
    }

    for map in &mut maps {
        map.sort_src.sort_by_key(|t| t.src);
    }

    (seeds, maps)
}

#[aoc_runner_derive::aoc(day5, part1)]
pub fn part1(input: &str) -> u64 {
    let (seeds, maps) = parse_input(input);

    seeds.map(|seed| maps.iter().fold(seed, |src, map| map.find_src_sorted(src))).min().unwrap()
}

#[aoc_runner_derive::aoc(day5, part2)]
pub fn part2(input: &str) -> u64 {
    let (seeds, maps) = parse_input(input);

    fn fold_tail_range<B>(
        range: Range,
        maps: &[Map],
        state: B,
        fold_fn: &mut impl FnMut(B, Range) -> B,
    ) -> B {
        if maps.is_empty() {
            fold_fn(state, range)
        } else {
            let (first, rest) = maps.split_first().unwrap();
            first.find_src_range_sorted(range).fold(state, |state, dest_range| {
                fold_tail_range(dest_range, rest, state, &mut *fold_fn)
            })
        }
    }

    seeds
        .array_chunks()
        .map(|[start, width]| Range { start, width })
        .map(|range| {
            fold_tail_range(range, &maps[..], u64::MAX, &mut |min, range| min.min(range.start))
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 46);
    }
}

use std::ops;

fn parse_line(mut line: &str) -> (u32, impl Iterator<Item = Set> + '_) {
    line = line.strip_prefix("Game ").expect("start with Game");

    let (id_str, sets) = line.split_once(':').expect("has colon");
    let id = id_str.parse::<u32>().unwrap();

    let sets = sets.split(';').map(|set| {
        set.split(',')
            .map(|mut term| {
                term = term.strip_prefix(' ').expect("space before number");
                let (num, color_str) = term.split_once(' ').expect("has space");
                let mut subset = Set::default();
                let color = match color_str {
                    "red" => &mut subset.r,
                    "green" => &mut subset.g,
                    "blue" => &mut subset.b,
                    _ => unreachable!(),
                };
                *color += num.parse::<u32>().unwrap();
                subset
            })
            .fold(Set::default(), |a, b| a + b)
    });
    (id, sets)
}

#[derive(Default)]
struct Set {
    r: u32,
    g: u32,
    b: u32,
}

impl Set {
    fn power(&self) -> u32 { self.r * self.g * self.b }
}

impl ops::Add for Set {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b }
    }
}

#[aoc_runner_derive::aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line)
        .filter_map(|(game, mut sets)| {
            sets.all(|set| set.r <= 12 && set.g <= 13 && set.b <= 14).then_some(game)
        })
        .sum()
}

#[aoc_runner_derive::aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line)
        .map(|(_, sets)| {
            sets.reduce(|a, b| Set { r: a.r.max(b.r), g: a.g.max(b.g), b: a.b.max(b.b) })
                .unwrap()
                .power()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 2286);
    }
}

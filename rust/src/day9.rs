use std::mem;

type InputSize = i32;
type ComputeSize = i64;

const FACTORIAL_MAX_WIDTH: usize = 13;
const FACTORIAL_SIZE: usize = 22;

struct FactorialSlice([[ComputeSize; FACTORIAL_MAX_WIDTH]; FACTORIAL_SIZE]);
impl Default for FactorialSlice {
    fn default() -> Self {
        let mut result = [[0; FACTORIAL_MAX_WIDTH]; FACTORIAL_SIZE];
        for i in 1..FACTORIAL_SIZE {
            let mut prod = 1 as ComputeSize;
            for j in (i + 1)..(i + FACTORIAL_MAX_WIDTH).min(FACTORIAL_SIZE) {
                prod *= j as ComputeSize;
                result[i][j - i] = prod;
                // self.0[x][y] stores (x+y)! / x!
            }
        }
        Self(result)
    }
}
impl FactorialSlice {
    /// Returns a! / b! / c!, where b < a && c < a && min(b, c) <= a/2
    fn get(&self, a: usize, mut b: usize, mut c: usize) -> ComputeSize {
        if b > c {
            mem::swap(&mut b, &mut c);
        }

        // b <= c < a
        // optimize this into (a! / c!) / b!

        let ac = if a == c { 1 } else { self.0[c][a - c] };
        let b_fact = if b <= 1 { 1 } else { self.0[1][b - 1] };

        return ac / b_fact;
    }
}

fn solve_1(fact: &FactorialSlice, line: &[InputSize]) -> ComputeSize {
    // Let P(n) be our polynomial such that P(i) = line[i] for i in 0..line.len()
    // Then by Lagrange polynomial interpolation, we have
    // P(n) = sum( (n! / (n - i)) / ( i! * (n-i-1)! * (-1)^(n-i-1) ) * line[i] for i in 0..n )
    // where n = line.len()

    line.iter()
        .enumerate()
        .map(|(i, &item)| {
            let mut coef =
                fact.get(line.len(), i, line.len() - i - 1) / ((line.len() - i) as ComputeSize);
            if (line.len() - i) % 2 == 0 {
                coef *= -1;
            }
            coef * item as ComputeSize
        })
        .sum::<ComputeSize>() as ComputeSize
}

fn solve_2(fact: &FactorialSlice, line: &[InputSize]) -> ComputeSize {
    // Similar to above, but we compute P(-1) instead of P(n)
    // P(n) = sum( (n! / (i+1) * (-1)^(n+1)) / ( i! * (n-i-1)! * (-1)^(n-i-1) ) * line[i] for i in 0..n )
    // where n = line.len()

    line.iter()
        .enumerate()
        .map(|(i, &item)| {
            let mut coef = fact.get(line.len(), i, line.len() - i - 1) / ((i + 1) as ComputeSize);
            if i % 2 == 1 {
                coef *= -1;
            }
            coef * item as ComputeSize
        })
        .sum::<ComputeSize>() as ComputeSize
}

fn solve(
    solver: impl Fn(&FactorialSlice, &[InputSize]) -> ComputeSize,
    input: &[u8],
) -> ComputeSize {
    let fact = FactorialSlice::default();

    let mut output = 0;

    let mut line: Vec<InputSize> = Vec::new();
    let mut current = 0;
    let mut negative = false;

    for &byte in input.iter().chain(b"\n") {
        match byte {
            b'-' => negative = true,
            b' ' | b'\n' => {
                line.push(current);
                current = 0;
                negative = false;

                if byte == b'\n' {
                    let result = solver(&fact, &line);
                    output += result;
                    line.clear();
                }
            }
            b'0'..=b'9' => {
                current *= 10;
                let digit = (byte - b'0') as InputSize;
                if negative {
                    current -= digit;
                } else {
                    current += digit;
                }
            }
            _ => unreachable!(),
        }
    }

    output
}

#[aoc_runner_derive::aoc(day9, part1)]
pub fn part1(input: &[u8]) -> ComputeSize { solve(solve_1, input) }

#[aoc_runner_derive::aoc(day9, part2)]
pub fn part2(input: &[u8]) -> ComputeSize { solve(solve_2, input) }

#[cfg(test)]
mod tests {
    use super::{ComputeSize, FactorialSlice};

    const SAMPLE: &[u8] = b"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_factorial() {
        let fact = FactorialSlice::default();
        for i in 16..=21 {
            for j in 5..13 {
                for k in 0..=3 {
                    let m = i - j - k;
                    assert_eq!(
                        fact.get(i as usize, j as usize, m as usize),
                        ((j + 1)..=i).product::<ComputeSize>() / (1..=m).product::<ComputeSize>()
                    );
                }
            }
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 18 + 28 + 68);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 2);
    }
}

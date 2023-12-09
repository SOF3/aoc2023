type InputSize = i32;
type ComputeSize = i128;

const FACTORIAL_SIZE: usize = 22;

struct Factorial([ComputeSize; FACTORIAL_SIZE]);
impl Default for Factorial {
    fn default() -> Self {
        let mut result = [0; FACTORIAL_SIZE];
        result[0] = 1;
        for i in 1..result.len() {
            result[i] = result[i - 1] * i as ComputeSize;
        }
        Self(result)
    }
}
impl Factorial {
    fn get(&self, i: usize) -> ComputeSize {
        self.0[i]
    }
}

fn solve_1(fact: &Factorial, line: &[InputSize]) -> ComputeSize {
    // Let P(n) be our polynomial such that P(i) = line[i] for i in 0..line.len()
    // Then by Lagrange polynomial interpolation, we have
    // P(n) = sum( (n! / (n - i)) / ( i! * (n-i-1)! * (-1)^(n-i-1) ) * line[i] for i in 0..n )
    // where n = line.len()

    let n_fact = fact.get(line.len()) as ComputeSize;
    line.iter()
        .enumerate()
        .map(|(i, &item)| {
            let mut numerator = n_fact / ((line.len() - i) as ComputeSize);
            if (line.len() - i) % 2 == 0 {
                numerator *= -1;
            }
            let denom1 = fact.get(i);
            let denom2 = fact.get(line.len() - i - 1);
            let coef = numerator / denom1 / denom2;
            coef * item as ComputeSize
        })
        .sum::<ComputeSize>() as ComputeSize
}

fn solve_2(fact: &Factorial, line: &[InputSize]) -> ComputeSize {
    // Similar to above, but we compute P(-1) instead of P(n)
    // P(n) = sum( (n! / (i+1) * (-1)^(n+1)) / ( i! * (n-i-1)! * (-1)^(n-i-1) ) * line[i] for i in 0..n )
    // where n = line.len()

    let n_fact = fact.get(line.len()) as ComputeSize;
    line.iter()
        .enumerate()
        .map(|(i, &item)| {
            let mut numerator = n_fact / ((i + 1) as ComputeSize);
            if i % 2 == 1 {
                numerator *= -1;
            }
            let denom1 = fact.get(i);
            let denom2 = fact.get(line.len() - i - 1);
            let coef = numerator / denom1 / denom2;
            coef * item as ComputeSize
        })
        .sum::<ComputeSize>() as ComputeSize
}

fn solve(solver: impl Fn(&Factorial, &[InputSize]) -> ComputeSize, input: &[u8]) -> ComputeSize {
    let fact = Factorial::default();

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
pub fn part1(input: &[u8]) -> ComputeSize {
    solve(solve_1, input)
}

#[aoc_runner_derive::aoc(day9, part2)]
pub fn part2(input: &[u8]) -> ComputeSize {
    solve(solve_2, input)
}

#[cfg(test)]
mod tests {
    const SAMPLE: &[u8] = b"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(SAMPLE), 18 + 28 + 68);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(SAMPLE), 2);
    }
}

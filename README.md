# aoc2023

Download personal inputs:

```bash
cargo install cargo-aoc
cd rust && cargo aoc input --all -y 2023
```

## Rust

Time to micro-optimize!
Solutions are written in idiomatic Rust with no unsafe code.

Run tests on sample data:

```bash
cd rust && cargo test day${DAY}
```

Run on personal inputs:

```bash
cd rust && cargo aoc -y2023 -d${DAY}
```

## JQ

JQ is more powerful than it sounds,
and it is a very crucial tool for those who
have to deal with debugging large JSON files every day.

Run tests on sample data:

```bash
cd jq && make test DAY=${DAY} PART=${PART}
```

Run on personal inputs:

```bash
cd jq && make run DAY=${DAY} PART=${PART}
```

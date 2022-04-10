# Avent of Code 2021

This is where I share my [Avent of Code](https://adventofcode.com)'s solution 😃

Language:

[![Rust](https://img.shields.io/badge/Rust%20and%20nothing%20more-ee7950.svg?style=for-the-badge&logo=rust)](#writteninrust)

## Usage:

```
cargo run --release -- <day>

<day>: AOC day

Flags: 
    -e, --example: Uses example file provided by AOC
    -a, --all: Gets all solutions for all days
```

For example: 

- `cargo run --release -- 1` to see solution for day 1
- `cargo run --release -- 1 --example (or -e)` to see solution for day 1 with example file
- `cargo run --release -- --all (or -a)` to see all solutions for all days

## Some note

- **Day1**: Everything looks good. Just need to use `window` and `zip` to get slice.

- **Day2**: Fastest as this time, most of the time used to get data. Use `Direction` enum instead of `&str` to avoid conversion. This makes increase speed so much. Wow!

- **Day3**: I still don't know why everything works well. It took me nearly two weeks of relaxing before facing this again. Working with `Binary` and `String` was an interesting and also a terrible experience. 

- **Day4**: All of mess

- **Day5**: 1000_000 fields... So slowwwwwwwwwwww

- **Day6**: I'm very confident about the first implement and my terminal nearly stop working as part2 starts.

- **Day7**: A lot of Math stuffs (nah, i don't know why they are very good at Math). It's fast as f*ck.

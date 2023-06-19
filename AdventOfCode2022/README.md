# Advent of Code 2022

Language:

![Swift](https://img.shields.io/badge/Swift%20and%20maybe%20more-ffffff.svg?style=for-the-badge&logo=swift)

## Prerequisite:

- Xcode >= 14
- Swift >= 5.9

## Setup:

1. `cd` into project folder on terminal

```bash
pod install
```

2. Open `AdventOfCode2022.xcodeproj` with `Xcode`

## Usage:

```bash
OVERVIEW: Advent of Code 2022

USAGE: aoc2022 <day> [--example] [--all]

ARGUMENTS:
  <day>                   AOC Day

OPTIONS:
  -e, --example           Uses example file provided by AOC
  -a, --all               Gets all solutions for all AOC days
  -h, --help              Show help information.
```

1. Go to menu `Product -> Scheme -> Edit Scheme`.
2. Look at `Arguments Passed On Launch` section on `Run` target.
3. Press `+` to add your wanted flag in order.
4. Close and run project.

For example:

- Add `1` to see solution for day 1
- Add another `-e` flag to see solution for day 1 with example file
- Add and tick only `-a` flag to see all solutions for all days

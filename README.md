# My solutions to [Advent of Code](https://adventofcode.com)

I have started solving the coding puzzles of
[Advent of Code](https://adventofcode.com)
in December 2022.
All solutions are written in Rust.

## Executing

This assumes that current Rust toolchain is installed.

1. Enter the directory of the appropriate year, e.g. `aoc2023`.
2. Place your input files in a new `input` directory with the name `dayX.txt`,
   where `X` is the name of the day the input belongs to.
   For example: `aoc2023/input/day1.txt`.
3. Run `cargo run --bin dayX` (again replacing `X` with the day number)
   to run the code on the given input.
4. Add `2` as a command line argument (`cargo run --bin dayX 2`)
   to run the second part of the puzzle.

Instead of relying on the `input` directory it is possible to directly specify
the path of the input file in the command line arguments.
For example: `cargo run --bin dayX 2 ../my_input.txt`.

> Note: Solutions of year 2022 are split into different binaries for each part
> of the puzzle. Use `--bin dayX-0` and `--bin dayX-1` in that case.

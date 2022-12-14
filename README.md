Advent of Code 2022
-------------------
[Advent of Code 2022](https://adventofcode.com/2022) written in rust.

Both parts for each day are included in the same project.

Expects input files to have Windows line endings (`\r\n`)

# Change Log

## Day 13
- Set logging level now uses `std::sync::Once` to avoid errors if called more than once

## Day 11
- To run part 2 with maximal [monkey business](https://www.youtube.com/watch?v=-g5cdzQIqJM) use flag `--part2`

## Day 6
- Takes a keyword argument `--size N` to set the lenght of the start message marker
    - Defaults to 4

## Day 3
- Add logging
    - `--test` sets logging level as `log::level::Debug`, otherwise `log::level::Info`

## Day 2
- Added utils -crate
- Added `--test` flag to run with `test_input.txt`

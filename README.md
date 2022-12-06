# Jazzinghen's AOC implementations - Rust

[![Rust](https://github.com/Jazzinghen/AdventOfCode/actions/workflows/rust.yml/badge.svg)](https://github.com/Jazzinghen/AdventOfCode/actions/workflows/rust.yml)

I am doing this in my free time whenever I have the energies. ~~Rust hates me.~~
You know what? It's actually not that bad once you know how to accept its
quirks (a.k.a. Stockholm's Syndrome).

## 2022

I took all the learnings from last year:

- Always use tests before debugging the main problem statement
- Use [nom][2] to parse input unless it's _very_ simple
- Iterators are your friends, even though it feels like they want you dead.
- About Iterators: use [itertools][3], for your own sanity

## 2021

I finished it! It was an uphill battle, but towards the end it became easier and
easier.

Also, you might want to build everything with `cargo build -r` to check the
actual speed of the solutions. As an example solution to day 23 went from
`6s 393ms 612µs` and `9s 564ms 869µs` in debug mode to `593ms 817µs` and
`678ms 579µs` in release mode.

## Advent of Code Rust Template

Advent of Code Rust template from [Replit's AoC templates][1], with some changes
to make the process smoother (and probably even make Clippy happy).

[1]: https://aoc-templates.util.repl.co/
[2]: https://github.com/Geal/nom
[3]: https://github.com/rust-itertools/itertools

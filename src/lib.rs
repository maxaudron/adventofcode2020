#![feature(exclusive_range_pattern)]
#![feature(or_patterns)]

#[macro_use]
extern crate aoc_runner_derive;

pub mod day1;
pub mod day2;
pub mod day3;
#[allow(unused_variables, dead_code)]
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub mod day10;

aoc_lib! { year = 2020 }

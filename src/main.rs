mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

extern crate core;
extern crate dotenv;

use std::env;
use std::time::Instant;

use clap::{arg, Parser};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    #[arg(value_parser = clap::value_parser!(i32).range(0..=25))]
    day: i32,

    /// Use test file instead (resources/day<day>.test.txt)
    #[arg(long, env, default_value_t = false)]
    test: bool,

    /// Measure execution time
    #[arg(short, long, env, default_value_t = false)]
    time: bool,
}
fn main() {
    let args = Args::parse();

    if args.test {
        env::set_var("TEST", "true");
    }

    if args.day == 0 {
        for d in 1..=25 {
            run(d, args.time);
            println!()
        }
    } else {
        run(args.day, args.time)
    }
}

fn run(day: i32, time: bool) {
    let start = Instant::now();
    match day {
        1 => day1::main(),
        2 => day2::main(),
        3 => day3::main(),
        4 => day4::main(),
        5 => day5::main(),
        6 => day6::main(),
        7 => day7::main(),
        8 => day8::main(),
        9 => day9::main(),
        10 => day10::main(),
        11 => day11::main(),
        12 => day12::main(),
        13 => day13::main(),
        14 => day14::main(),
        15 => day15::main(),
        16 => day16::main(),
        17 => day17::main(),
        18 => day18::main(),
        19 => day19::main(),
        20 => day20::main(),
        21 => day21::main(),
        22 => day22::main(),
        23 => day23::main(),
        24 => day24::main(),
        25 => day25::main(),
        other => {
            println!("Day {} not yet implemented 😅", other)
        }
    }
    let duration = start.elapsed();
    if time {
        if duration.as_millis() < 3 {
            println!("Time: {} μs", duration.as_micros())
        } else {
            println!("Time: {} ms", duration.as_millis())
        }
    }
}

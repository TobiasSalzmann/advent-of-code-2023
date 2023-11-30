mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day17;
mod day18;
mod day16;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

extern crate dotenv;
extern crate core;

use dotenv::dotenv;
use std::env;
use std::time::Instant;

fn main() {
    let day_string = env::args().nth(1).or_else(|| {
        dotenv().ok();
        env::var("DAY").ok()
    }).unwrap_or("1".to_string());

    let day = day_string.parse::<i32>().expect("Wrong format for day variable");

    if day == 0 {
        for d in 1..=25 {
            run(d, env::var("TIME").is_ok());
            println!()
        }
    } else {
        run(day, env::var("TIME").is_ok())
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
        other => { println!("Day {} not yet implemented 😅", other) }
    }
    let duration = start.elapsed();
    if time {
        println!("Time: {} ms", duration.as_millis())
    }
}

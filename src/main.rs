mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod util;
mod day7;

extern crate core;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::time::Instant;

fn main() {
    let day_string = env::args()
        .nth(1)
        .or_else(|| {
            dotenv().ok();
            env::var("DAY").ok()
        })
        .unwrap_or("1".to_string());

    let day = day_string
        .parse::<i32>()
        .expect("Wrong format for day variable");

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
        other => {
            println!("Day {} not yet implemented 😅", other)
        }
    }
    let duration = start.elapsed();
    if time {
        println!("Time: {} ms", duration.as_millis())
    }
}

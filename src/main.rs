mod day1;
mod util;

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
        other => {
            println!("Day {} not yet implemented 😅", other)
        }
    }
    let duration = start.elapsed();
    if time {
        println!("Time: {} ms", duration.as_millis())
    }
}

use std::env;

use clap::Parser;

mod day1;
mod day2;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    day: u32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    //day1::solve();
    //day1::solve_pt2();
    day2::solve();
}


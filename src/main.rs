extern crate clap;

use clap::{crate_version, App, Arg};
use std::io::{self, BufRead};
mod math;

type MathFunction = for<'r> fn(&'r Vec<f64>) -> f64;

enum MathFunctionLabel {
    Sum,
    Mean,
}

fn parse_args() -> MathFunctionLabel {
    let matches = App::new("pipemath")
        .about(
            "Run math functions on numbers from stdin.

Takes all input from stdin and applies a statistical function
It ignores invalid input.",
        )
        .version(crate_version!())
        .arg(
            Arg::with_name("FUNCTION")
                .help("What function to compute")
                .index(1)
                .possible_values(&["mean", "sum"])
                .required(true),
        )
        .get_matches();

    // Note, it's safe to call unwrap() because the arg is required
    match matches.value_of("FUNCTION").unwrap() {
        "mean" => MathFunctionLabel::Mean,
        "sum" => MathFunctionLabel::Sum,
        _ => unreachable!(),
    }
}

fn main() {
    let funcname = parse_args();
    let mut nums: Vec<f64> = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Err(e) => eprintln!("ERROR reading stdin: {}", e), // with ^Z
            Ok(s) => match s.as_str().parse() {
                Err(e) => eprintln!("ERROR cannot parse '{}': {}", s, e),
                Ok(f) => nums.push(f),
            },
        }
    }
    if nums.len() == 0 {
        std::process::exit(1);
    }
    let func: MathFunction;
    match funcname {
        MathFunctionLabel::Mean => func = math::mean,
        MathFunctionLabel::Sum => func = math::sum,
    }
    println!("{}", func(&nums));
    std::process::exit(0);
}

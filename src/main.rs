extern crate aoc_2019;

use aoc_2019::fuel_requirements;
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    // read all lines
    let a: Vec<String> = args().collect();
    let mut f = File::open(&a[1]).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let sum = contents.lines().fold(0.0, |acc, input| {
        acc + fuel_requirements(input.parse::<f64>().unwrap())
    });

    println!("{}", sum);
}

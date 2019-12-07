extern crate aoc_2019;

use aoc_2019::{intcode_program, parse_intcode_input};
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let a: Vec<String> = args().collect();
    let mut f = File::open(&a[1]).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let opcodes = parse_intcode_input(contents);
    // brute force is still cheap...
    for i in 0..100 {
        for j in 0..100 {
            let mut opcodes_copy = opcodes.clone();
            opcodes_copy[1] = i;
            opcodes_copy[2] = j;
            let result = intcode_program(opcodes_copy);
            if result[0] == 19690720 {
                println!("{} - {}", i, j);
                break;
            }
        }
    }
}

extern crate aoc_2019;

use aoc_2019::manhattan_distance_of_closest_intersection;
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let a: Vec<String> = args().collect();
    let mut f = File::open(&a[1]).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let mut l = contents.lines();
    let m = manhattan_distance_of_closest_intersection(
        l.next().unwrap().to_string(),
        l.next().unwrap().to_string(),
    );
    println!("{}", m);
}

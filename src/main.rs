extern crate aoc_2019;

use aoc_2019::count_viable_passwords_in_range;

fn main() {
    let m = count_viable_passwords_in_range(123257, 647015);
    println!("{}", m);
}

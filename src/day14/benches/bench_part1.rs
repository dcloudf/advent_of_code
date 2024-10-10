use day14::part1::calc_total_load;
use divan::black_box;
use std::fs::read_to_string;

fn main() {
    divan::main()
}

#[divan::bench]
fn bench_part1() {
    let _ = calc_total_load(black_box(read_to_string("input_data.txt").unwrap()));
}

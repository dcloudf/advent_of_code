use day14::part1::calc_total_load;

fn main() {
    let input = include_str!("../input_data.txt").to_string();
    println!("{}", calc_total_load(input));
}

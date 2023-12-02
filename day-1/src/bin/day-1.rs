use day_1::trebuchet;

fn main() {
    let data = include_str!("../../data/day1.txt");
    let result = trebuchet::process(data);
    println!("{result}");
}

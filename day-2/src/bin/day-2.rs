use day_2::cube_conundrum;

fn main() {
    let data = include_str!("../../data/day2.txt");
    let result = cube_conundrum::process(data);    
    println!("{result}")
}

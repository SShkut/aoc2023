use day_2::cube_conundrum;

fn main() {
    let data = include_str!("../../data/day2.txt");
    let result_day1 = cube_conundrum::process_day1(data);    
    let result_day2 = cube_conundrum::process_day2(data);    
    println!("{}", result_day1);
    println!("{}", result_day2);
}

#[derive(Debug)]
struct Position {
    y: i32,
    x: i32,
    value: char
}

pub fn process_day1(data: &str) -> u32 {
    let characters_matrix: Vec<Vec<Position>> = data
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, value)| Position {
                    y: y as i32,
                    x: x as i32,
                    value,
                })
                .collect()
        })
        .collect();
        println!("{:?}", characters_matrix);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process_day1(&data);
        let expected = 467 + 35 + 633 + 617 + 592 + 755 + 664 + 598;
        assert_eq!(expected, result);
    }
}

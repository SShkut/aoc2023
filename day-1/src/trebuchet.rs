pub fn process(data: &str) -> u32 {
    data.lines()
        .map(extract_calibration_values)
        .sum()
}

fn extract_calibration_values(line: &str) -> u32 {
    let mut numbers = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            Some(1)
        } else if reduced_line.starts_with("two") {
            Some(2)
        } else if reduced_line.starts_with("three") {
            Some(3)
        } else if reduced_line.starts_with("four") {
            Some(4)
        } else if reduced_line.starts_with("five") {
            Some(5)
        } else if reduced_line.starts_with("six") {
            Some(6)
        } else if reduced_line.starts_with("seven") {
            Some(7)
        } else if reduced_line.starts_with("eight") {
            Some(8)
        } else if reduced_line.starts_with("nine") {
            Some(9)
        } else {
            reduced_line.chars().next().unwrap().to_digit(10)
        };

        result
    });
    let first = numbers.next().unwrap_or(0); 

    match numbers.last() {
        Some(number) => first * 10 + number,
        None => first * 10 + first,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_process_part1() {
        let data = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let expected = 12 + 38 + 15 + 77;
        let result = process(&data);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_process_part2() {
        let data = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
onethreenfkgrvsevenkczctlgkt7
";
        let expected = 29 + 83 + 13 + 24 + 42 + 14 + 76 + 17;
        let result = process(&data);
        assert_eq!(expected, result);
    }
    #[test]
    fn test_process_empty() {
        let data = "";
        let expected = 0;
        let result = process(&data);
        assert_eq!(expected, result);
    }
}

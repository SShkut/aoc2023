pub fn process(data: &str) -> u32 {
    data.lines().map(extract_calibration_values).sum()
}

fn extract_calibration_values(line: &str) -> u32 {
    let numbers: String = line.chars().filter(|c| c.is_numeric()).collect();
    let first = numbers.chars().next().unwrap_or('0');
    let last = numbers.chars().last().unwrap_or('0');
    format!("{}{}", first, last).parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let data = "3two9eight7dl5\n\
            6five2thone65\n\
            fsevensix916\n\
            nejzbpzone7\n";
        let expected = 35 + 65 + 96 + 77;
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

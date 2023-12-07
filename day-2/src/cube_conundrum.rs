use std::collections::BTreeSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug)]
struct Round {
    cubes: Vec<Cube>,
}

impl Round {
    fn calculate_round(&self) -> Vec<Cube> {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        self.cubes.iter().for_each(|cube| match cube {
            Cube::Red(value) => red += value,
            Cube::Green(value) => green += value,
            Cube::Blue(value) => blue += value,
        });
        vec![Cube::Red(red), Cube::Blue(blue), Cube::Green(green)]
    }
}

#[derive(Debug)]
struct Game<'a> {
    id: &'a str,
    rounds: Vec<Round>,
}

impl<'a> Game<'a> {
    fn is_game_valid(&'a self, threshold: &BTreeSet<Cube>) {}
}

pub fn process(data: &str) -> u32 {
    let (_, games) = parse_game(data).expect("Should parse.");
    let threshold = BTreeSet::from([Cube::Red(12), Cube::Green(13), Cube::Blue(14)]);
    0
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    let cube = match color {
        "red" => Cube::Red(amount),
        "green" => Cube::Green(amount),
        "blue" => Cube::Blue(amount),
        _ => unreachable!(),
    };
    Ok((input, cube))
}

fn round(input: &str) -> IResult<&str, Round> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
    Ok((input, Round { cubes }))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;
    Ok((input, Game { id, rounds }))
}

fn parse_game(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

#[cfg(test)]
mod tests {
    use super::*;

    //12 red cubes, 13 green cubes, and 14 blue cubes
    #[test]
    fn test_process_part1() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = 1 + 2 + 5;
        let result = process(&data);
        assert_eq!(expected, result);
    }
}

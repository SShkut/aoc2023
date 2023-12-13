use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Not,
    str::FromStr,
};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cube {
    color: Color,
    amount: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "red" | "Red" => Ok(Color::Red),
            "green" | "Green" => Ok(Color::Green),
            "blue" | "Blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Round {
    cubes: Vec<Cube>,
}

impl Round {
    fn calculate_round(&self) -> BTreeSet<Cube> {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        self.cubes.iter().for_each(|cube| match cube.color {
            Color::Red => red += cube.amount,
            Color::Green => green += cube.amount,
            Color::Blue => blue += cube.amount,
        });
        BTreeSet::from([
            Cube {
                color: Color::Red,
                amount: red,
            },
            Cube {
                color: Color::Green,
                amount: green,
            },
            Cube {
                color: Color::Blue,
                amount: blue,
            },
        ])
    }
}

#[derive(Debug)]
struct Game<'a> {
    id: &'a str,
    rounds: Vec<Round>,
}

impl<'a> Game<'a> {
    fn get_valid_game_id(&'a self, threshold: &BTreeMap<Color, u32>) -> Option<u32> {
        self.rounds
            .iter()
            .flat_map(|round| round.calculate_round())
            .any(|cube| {
                cube.amount
                    > *threshold
                        .get(&cube.color)
                        .expect("Should be a correct color")
            })
            .not()
            .then_some(self.id.parse::<u32>().expect("Id should be parsable"))
    }
}

pub fn process_day1(data: &str) -> u32 {
    let (_, games) = parse_game(data).expect("Should parse.");
    let threshold = BTreeMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

    games
        .iter()
        .filter_map(|game| game.get_valid_game_id(&threshold))
        .sum()
}

pub fn process_day2(data: &str) -> u32 {
    let (_, games) = parse_game(data).expect("Sould parse.");
    let mut fewest_score: Vec<u32> = Vec::new();
    for game in games {
        let mut red_max = 1;
        let mut blue_max = 1;
        let mut green_max = 1;
        for round in game.rounds {
            for cube in round.cubes {
                match cube.color {
                    Color::Blue => blue_max = blue_max.max(cube.amount),
                    Color::Red => red_max = red_max.max(cube.amount),
                    Color::Green => green_max = green_max.max(cube.amount),
                }
            }
        }
        fewest_score.push(red_max * green_max * blue_max);
    }

    fewest_score.into_iter().sum()
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((
        input,
        Cube {
            color: Color::from_str(color).unwrap(),
            amount,
        },
    ))
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
        let result = process_day1(&data);
        assert_eq!(expected, result);
    }
}

#[test]
fn test_process_part2() {
    let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let first_game = 4 * 2 * 6;
    let second_game = 4 * 3 * 1;
    let third_game = 20 * 13 * 6;
    let fourth_geme = 14 * 3 * 15;
    let fifth_game = 6 * 3 * 2;
    let expected = first_game + second_game + third_game + fourth_geme + fifth_game;
    let result = process_day2(&data);
    assert_eq!(expected, result);
}

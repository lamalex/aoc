use std::str::FromStr;

use nom::{IResult, bytes::complete::tag_no_case, character::complete::{u32, space1, line_ending}, sequence::{preceded, tuple, terminated}, branch::alt, multi::separated_list1};
use strum::{Display, EnumString};

fn main() {
    let reference_pull = Pull::new(Some(Red(12)), Some(Blue(14)), Some(Green(13)));

    let (_, games) = parse_games(include_str!("input.txt")).unwrap();

    let sum_of_ids = games.iter()
            .filter(|g| g.pulls.iter().all(|p| p.cmp(&reference_pull) != std::cmp::Ordering::Greater))
            .map(|g| g.id)
            // .collect::<Vec<u32>>();
            .sum::<u32>();
    println!("{sum_of_ids:?}");

    let power = games.iter()
        .map(|g| g.pulls.iter()
            .fold(Pull::default(), |max, next| Pull::new(
                Some(Red(max.red.max(next.red))),
                Some(Blue(max.blue.max(next.blue))),
                Some(Green(max.green.max(next.green))),
            ))
        )
        .map(|p| p.red * p.blue * p.green)
        .sum::<u32>();

    println!("{power}");
}

impl Game{
    #[must_use]
    pub fn new(id: u32, pulls: Vec<Pull>) -> Self {
        Game {
            id,
            pulls,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd)]
struct Pull {
    red: u32,
    blue: u32,
    green: u32,
}

impl Pull {
    #[must_use]
    pub fn new(red: Option<Red>, blue: Option<Blue>, green: Option<Green>) -> Self {
        Self {
            red: red.map(|r| r.0).unwrap_or_default(),
            blue: blue.map(|b| b.0).unwrap_or_default(),
            green: green.map(|g| g.0).unwrap_or_default(),
        }
    }
}

impl Ord for Pull {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.red == other.red && self.blue == other.blue && self.green == other.green {
            std::cmp::Ordering::Equal
        }
        else if self.red <= other.red && self.blue <= other.blue && self.green <= other.green {
            std::cmp::Ordering::Less
        }
        else {
            std::cmp::Ordering::Greater
        }
    }
}


pub fn parse_game_id(i: &str) -> IResult<&str, u32> {
    preceded(tag_no_case("game "), u32)(i)
}

fn parse_color(i: &str) -> IResult<&str, Color> {
    nom::combinator::map(alt((tag_no_case("red"), tag_no_case("green"), tag_no_case("blue"))), |tag: &str| {
        Color::from_str(tag).unwrap()
    })(i)
}

fn parse_pull_part(i: &str) -> IResult<&str, (u32, Color)> {
    tuple((terminated(u32, space1), parse_color))(i)
}

fn parse_full_pull(i: &str) -> IResult<&str, Pull> {
    nom::combinator::map(separated_list1(tag_no_case(", "), parse_pull_part), |pulls| {
        pulls.into_iter()
            .fold(Pull::default(), |mut pull, (count, color)| {
                match color {
                    Color::Red => pull.red = count,
                    Color::Blue => pull.blue = count,
                    Color::Green => pull.green = count,
                }

                pull
            })
    })(i)
}

fn parse_game(i: &str) -> IResult<&str, Game> {
    nom::combinator::map(
        tuple((
            terminated(parse_game_id, tag_no_case(": ")),
            separated_list1(tag_no_case("; "), parse_full_pull)
        )),
        |(id, pulls)| {
            Game::new(id, pulls)
        }
    )(i)
}

fn parse_games(i: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, parse_game)(i)
}

#[derive(Debug, Clone, PartialEq, Eq, Display, EnumString)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "lowercase")]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Red(u32);
#[derive(Debug, Clone, PartialEq, Eq)]
struct Green(u32);
#[derive(Debug, Clone, PartialEq, Eq)]
struct Blue(u32);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Game {
    id: u32,
    pulls: Vec<Pull>
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use test_case::test_case;


    fn color_enum_strateg() -> impl Strategy<Value = Color> {
        prop_oneof![
            Just(Color::Red),
            Just(Color::Green),
            Just(Color::Blue)
        ]
    }

    proptest! {
        #[test]
        fn test_parse_game_id(expected_id in 0u32..u32::MAX) {
            let line = format!("Game {expected_id}");

            let (_, actual_id) = parse_game_id(&line).unwrap();
            assert_eq!(expected_id, actual_id);
        }
    }

    proptest! {
        #[test]
        fn test_parse_color(expected_color in color_enum_strateg()) {
            let (_, actual_color) = parse_color(&expected_color.to_string()).unwrap();
            assert_eq!(expected_color, actual_color);
        }
    }

    proptest! {
        #[test]
        fn test_parse_dice_pull(expected_pull_count in 0u32..u32::MAX, expected_color in color_enum_strateg()) {
            let line = format!("{expected_pull_count} {}", expected_color);
            let (_, (actual_pull_count, actual_color)) = parse_pull_part(&line).unwrap();

            assert_eq!(expected_pull_count, actual_pull_count);
            assert_eq!(expected_color, actual_color);
        }
    }

    #[test_case("3 blue, 4 red", Pull::new(Some(Red(4)), Some(Blue(3)), None))]
    #[test_case("1 red, 2 green, 6 blue", Pull::new(Some(Red(1)), Some(Blue(6)), Some(Green(2))))]
    #[test_case("2 green", Pull::new(None, None, Some(Green(2))))]
    fn test_parse_full_pull(line: &str, expected_pull: Pull) {
        let (_, actual_pull) = parse_full_pull(line).unwrap();
        assert_eq!(expected_pull, actual_pull);
    }

    #[test_case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 
        Game::new(
            1, 
            vec![
                Pull::new(Some(Red(4)), Some(Blue(3)), None), 
                Pull::new(Some(Red(1)), Some(Blue(6)), Some(Green(2))),
                Pull::new(None, None, Some(Green(2)))
            ]
        )
    )]
    #[test_case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        Game::new(
            2,
            vec![
                Pull::new(None, Some(Blue(1)), Some(Green(2))),
                Pull::new(Some(Red(1)), Some(Blue(4)), Some(Green(3))),
                Pull::new(None, Some(Blue(1)), Some(Green(1)))
            ]
        )
    )]
    #[test_case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        Game::new(
            3,
            vec![
                Pull::new(Some(Red(20)), Some(Blue(6)), Some(Green(8))),
                Pull::new(Some(Red(4)), Some(Blue(5)), Some(Green(13))),
                Pull::new(Some(Red(1)), None, Some(Green(5)))
            ]
        )
    )]
    #[test_case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        Game::new(
            4,
            vec![
                Pull::new(Some(Red(3)), Some(Blue(6)), Some(Green(1))),
                Pull::new(Some(Red(6)), None, Some(Green(3))),
                Pull::new(Some(Red(14)), Some(Blue(15)), Some(Green(3)))
            ]
        )
    )]
    #[test_case(
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        Game::new(
            5,
            vec![
                Pull::new(Some(Red(6)), Some(Blue(1)), Some(Green(3))),
                Pull::new(Some(Red(1)), Some(Blue(2)), Some(Green(2)))
            ]
        )
    )]
    fn test_parse_game(line: &str, expected_game: Game) {
        let (_, actual_game) = parse_game(line).unwrap();
        assert_eq!(expected_game, actual_game);

    }

    #[test]
    fn test_parse_series_of_games() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green        
"#;
        let (_, games) = parse_games(input).unwrap();

        let expected_games = vec![
            Game::new(
                1, 
                vec![
                    Pull::new(Some(Red(4)), Some(Blue(3)), None), 
                    Pull::new(Some(Red(1)), Some(Blue(6)), Some(Green(2))),
                    Pull::new(None, None, Some(Green(2)))
                ]
            ),
            Game::new(
                2,
                vec![
                    Pull::new(None, Some(Blue(1)), Some(Green(2))),
                    Pull::new(Some(Red(1)), Some(Blue(4)), Some(Green(3))),
                    Pull::new(None, Some(Blue(1)), Some(Green(1)))
                ]
            ),
            Game::new(
                3,
                vec![
                    Pull::new(Some(Red(20)), Some(Blue(6)), Some(Green(8))),
                    Pull::new(Some(Red(4)), Some(Blue(5)), Some(Green(13))),
                    Pull::new(Some(Red(1)), None, Some(Green(5)))
                ]
            ),
            Game::new(
                4,
                vec![
                    Pull::new(Some(Red(3)), Some(Blue(6)), Some(Green(1))),
                    Pull::new(Some(Red(6)), None, Some(Green(3))),
                    Pull::new(Some(Red(14)), Some(Blue(15)), Some(Green(3)))
                ]
            ),
            Game::new(
                5,
                vec![
                    Pull::new(Some(Red(6)), Some(Blue(1)), Some(Green(3))),
                    Pull::new(Some(Red(1)), Some(Blue(2)), Some(Green(2)))
                ]
            ),
        ];

        assert_eq!(games, expected_games);

        let reference_pull = Pull::new(Some(Red(12)), Some(Blue(14)), Some(Green(13)));
        let sum_of_ids: u32 = games.iter()
            .filter(|g| g.pulls.iter().all(|p| p <= &reference_pull))
            .map(|g| g.id)
            .sum();

        assert_eq!(8, sum_of_ids);

    }
}

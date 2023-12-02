use nom::{
    bytes::complete::{tag, take_till},
    character::complete::digit1,
    multi::separated_list1,
    Err, IResult, Needed,
};
#[derive(PartialEq, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

struct ColourCombination {
    red: i32,
    green: i32,
    blue: i32,
}

impl ColourCombination {
    fn from_vec(color_vec: Vec<(Color, i32)>) -> Self {
        let (mut red, mut blue, mut green) = (0, 0, 0);
        for (color, value) in color_vec.iter() {
            match color {
                Color::Red => {
                    red = value.clone();
                }
                Color::Blue => {
                    blue = value.clone();
                }
                Color::Green => {
                    green = value.clone();
                }
            }
        }

        ColourCombination { red, green, blue }
    }
}

struct Game<'a> {
    number: &'a str,
    color_combos: Vec<ColourCombination>,
}

impl<'a> Game<'a> {
    fn is_possible(&self) -> (bool, i32) {
        let (mut max_red, mut max_blue, mut max_green) = (0, 0, 0);
        for color_c in self.color_combos.iter() {
            if color_c.red > max_red {
                max_red = color_c.red;
            }
            if color_c.blue > max_blue {
                max_blue = color_c.blue;
            }
            if color_c.green > max_green {
                max_green = color_c.green;
            }
        }

        if max_red > 12 || max_blue > 14 || max_green > 13 {
            return (false, 0);
        }
        (true, max_red * max_blue * max_green)
    }
    fn cube_power(&self) -> i32 {
        let (mut max_red, mut max_blue, mut max_green) = (0, 0, 0);
        for color_c in self.color_combos.iter() {
            if color_c.red > max_red {
                max_red = color_c.red;
            }
            if color_c.blue > max_blue {
                max_blue = color_c.blue;
            }
            if color_c.green > max_green {
                max_green = color_c.green;
            }
        }

        max_red * max_blue * max_green
    }
}

fn till_comma_or_semicolor(s: &str) -> IResult<&str, &str> {
    take_till(|c| c == ',' || c == ';')(s)
}

fn parse_colour(input: &str) -> IResult<&str, (Color, i32)> {
    // 3 red
    // 11 green
    let (input, num) = digit1(input)?;
    let (input, mut colour_name) = till_comma_or_semicolor(input)?;
    colour_name = colour_name.trim();
    if colour_name == "red" {
        Ok((input, (Color::Red, num.parse().unwrap())))
    } else if colour_name == "blue" {
        Ok((input, (Color::Blue, num.parse().unwrap())))
    } else if colour_name == "green" {
        Ok((input, (Color::Green, num.parse().unwrap())))
    } else {
        println!("Unknown: {}", input);
        Err(Err::Incomplete(Needed::new(4)))
    }
}

fn parse_colours(input: &str) -> IResult<&str, ColourCombination> {
    // 3 red, 11 green, 2 blue
    // 3 blue, 6 green
    let (input, num_color) = separated_list1(tag(", "), parse_colour)(input)?;

    Ok((input, ColourCombination::from_vec(num_color)))
}

#[test]
fn test_parse_colour() {
    let (_, (colour, score)) = parse_colour("3 red").unwrap();
    assert_eq!(colour, Color::Red);
    assert_eq!(score, 3);
}

#[test]
fn test_parse_colours() {
    let (_, colour_comb) = parse_colours("3 red, 11 green, 2 blue").unwrap();
    assert_eq!(colour_comb.blue, 2);
    assert_eq!(colour_comb.red, 3);
    assert_eq!(colour_comb.green, 11);
    let (_, colour_comb) = parse_colours("3 red, 2 blue").unwrap();
    assert_eq!(colour_comb.blue, 2);
    assert_eq!(colour_comb.red, 3);
    assert_eq!(colour_comb.green, 0);
}

// Game 85: 3 red, 11 green, 2 blue; 3 blue, 6 green; 2 red, 4 green, 4 blue; 1 blue, 3 red, 10 green; 4 blue, 7 green, 4 red
fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, game_number) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, color_combinations) = separated_list1(tag("; "), parse_colours)(input)?;
    let game = Game {
        number: game_number,
        color_combos: color_combinations,
    };
    Ok((input, game))
}

#[test]
fn test_parse_game() {
    let (_, (game)) = parse_game("Game 85: 3 red, 11 green, 2 blue; 3 blue, 6 green; 2 red, 4 green, 4 blue; 1 blue, 3 red, 10 green; 4 blue, 7 green, 4 red").unwrap();
    assert_eq!(game.color_combos.len(), 5);
    assert_eq!(game.number, "85");
}

fn main() {
    let lines = include_str!("input.txt").lines();
    let mut answer: i32 = 0;
    let mut answer_2: i32 = 0;
    for ln in lines {
        let (_, game) = parse_game(ln).unwrap();
        let (is_poss, _) = game.is_possible();
        if is_poss {
            answer += game.number.parse::<i32>().unwrap();
        }
        answer_2 += game.cube_power();
    }
    println!("Part 1 answer: {}", answer);
    println!("Part 2 answer: {}", answer_2);
}

use color::Color;
use cost::Tokens;

fn create_card(
    color: Color,
    points: u8,
    black: u8,
    blue: u8,
    green: u8,
    red: u8,
    white: u8,
) -> Card {
    Card {
        color: color,
        cost: Tokens {
            black: black,
            blue: blue,
            green: green,
            red: red,
            white: white,
            joker: 0,
        },
        points: points,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub color: Color,
    pub cost: Tokens,
    pub points: u8,
}

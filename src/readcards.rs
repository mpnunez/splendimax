use std::{error::Error, io, process};

use color::Color;
use cost::Tokens;
use card::Card;



// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, serde::Deserialize)]
struct CardRecord {
    #[serde(rename = "Level")]
    level: u8,
    #[serde(rename = "Color")]
    color: String,
    #[serde(rename = "PV")]
    pv: u8,
    #[serde(rename = "Black")]
    black: u8,
    #[serde(rename = "Blue")]
    blue: u8,
    #[serde(rename = "Green")]
    green: u8,
    #[serde(rename = "Red")]
    red: u8,
    #[serde(rename = "White")]
    white: u8,
}

impl CardRecord {

    fn get_color(&self) -> Color {
        match self.color.as_ref() {
            "Black" => Color::Black,
            "Blue" => Color::Blue,
            "Green" => Color::Green,
            "Red" => Color::Red,
            "White" => Color::White,
            _ => panic!("Unknown color {}", self.color),
        }
    }

    fn create_card(&self) -> Card {
        Card {
            color: self.get_color(),
            cost: Tokens {
                black: self.black,
                blue: self.blue,
                green: self.green,
                red: self.red,
                white: self.white,
                joker: 0,
            },
            points: self.pv,
        }
    }
}

pub fn read_cards(fname: &str) -> Vec<Vec<Card>> {

    // 3 empty decks to be populated
    let mut decks: Vec<Vec<Card>> = Vec::new();
    for i in 1..4 {
        decks.push(Vec::<Card>::new());
    }

    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(fname).expect("cards.csv not read");
    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        if result.is_err() {
            continue;
        }
        let record: CardRecord = result.unwrap();
        let card = record.create_card();
        let ind = record.level - 1;
        decks[usize::from(ind)].push(card.clone());
        println!("{:?}", card);
    }

    decks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_decks () {
        let decks = read_cards("cards.csv");
        let deck1 = decks[0].clone();
        assert_eq!(deck1.len(),40);
        let deck1 = decks[1].clone();
        assert_eq!(deck1.len(),30);
        let deck1 = decks[2].clone();
        assert_eq!(deck1.len(),20);
    }

}
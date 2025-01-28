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
            _ => Color::Joker,
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

pub fn read_cards_of_level(fname: &str, level: u8) -> Vec<Card> {

    // 3 empty decks to be populated
    let mut deck: Vec<Card> = Vec::new();

    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(fname).expect("cards.csv not read");
    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        if result.is_err() {
            continue;
        }
        let record: CardRecord = result.unwrap();
        if record.level != level {
            continue;
        }
        let card = record.create_card();
        deck.push(card.clone());
    }

    deck
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_decks () {
        let deck1 = read_cards_of_level("cards.csv",1);
        assert_eq!(deck1.len(),40);
        let deck2 = read_cards_of_level("cards.csv",2);
        assert_eq!(deck2.len(),30);
        let deck3 = read_cards_of_level("cards.csv",3);
        assert_eq!(deck3.len(),20);
        let nobles = read_cards_of_level("cards.csv",0);
        assert_eq!(nobles.len(),9);
    }

}
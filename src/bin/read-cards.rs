use std::{error::Error, io, process};

extern crate splendimax;
use splendimax::color::Color;
use splendimax::cost::Tokens;
use splendimax::card::Card;



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

fn main() {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path("cards.csv").expect("cards.csv not read");
    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        if result.is_err() {
            continue;
        }
        let record: CardRecord = result.unwrap();
        let card = record.create_card();
        println!("{:?}", card);
    }
}

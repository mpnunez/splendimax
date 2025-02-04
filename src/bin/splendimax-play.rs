extern crate rand;
extern crate splendimax;

use rand::{thread_rng, Rng};
use splendimax::algo::alphabeta;
use splendimax::algo::state::Score;
use splendimax::algo::state::State as AlgoState;
use splendimax::card::Card;
use splendimax::color::Color;
use splendimax::cost::Tokens;
use splendimax::state::{Deck, Move, State};
use std::io;

fn main() {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();
    let mut state = State::new(2);
    let mut rng = thread_rng();
    let mut round = 0;
    state.print(&mut stdout);
    loop {
        if state.is_terminal() {
            state.print(&mut stdout);
            break;
        }

        if state.players_turn {
            let moves = alphabeta(&mut state);
            if let Some(mov) = rng.choose(&moves) {
                println!("{:?}", mov);
                state.apply(&mov);
            } else {
                state.print(&mut stdout);
                panic!("No moves");
            }
            round += 1;
        } else {
            state.print(&mut stdout);
            'outer: loop {
                let mut buf = String::new();
                println!("Please specify action. (t)ake rkw, (b)uy 1 3 (row column), (r)eserve 1 3 (row column) b(u)y reserved 1 (index), (p)ass");
                let mov;
                match stdin.read_line(&mut buf) {
                    Ok(_) => {
                        let mut iter = buf.chars();
                        match iter.next() {
                            Some('t') => {
                                let mut tokens = Tokens::empty();
                                for c in iter {
                                    match c {
                                        'k' => tokens.black += 1,
                                        'b' => tokens.blue += 1,
                                        'g' => tokens.green += 1,
                                        'r' => tokens.red += 1,
                                        'w' => tokens.white += 1,
                                        _ => continue,
                                    }
                                }
                                mov = Move::Take {
                                    tokens: tokens,
                                    drop: Tokens::empty(),
                                };
                            }
                            Some('b') => {
                                let cards;
                                let deck;
                                loop {
                                    let c = iter.next();
                                    match c {
                                        Some('1') => {
                                            cards = &state.cards1;
                                            deck = Deck::One;
                                            break;
                                        }
                                        Some('2') => {
                                            cards = &state.cards2;
                                            deck = Deck::Two;
                                            break;
                                        }
                                        Some('3') => {
                                            cards = &state.cards3;
                                            deck = Deck::Three;
                                            break;
                                        }
                                        Some(_) => continue,
                                        None => {
                                            println!("invalid command");
                                            break 'outer;
                                        }
                                    }
                                }

                                let index: u8;
                                loop {
                                    let c = iter.next();
                                    match c {
                                        Some('1') => {
                                            index = 0;
                                            break;
                                        }
                                        Some('2') => {
                                            index = 1;
                                            break;
                                        }
                                        Some('3') => {
                                            index = 2;
                                            break;
                                        }
                                        Some('4') => {
                                            index = 3;
                                            break;
                                        }
                                        Some(_) => continue,
                                        None => {
                                            println!("invalid command");
                                            break 'outer;
                                        }
                                    }
                                }

                                if let Some(ref card) = cards.get(index as usize) {
                                    if let Some(cost) = state.adversary.cost_for(card) {
                                        let mut tokens_from_cards =
                                            state.adversary.tokens_from_cards();
                                        tokens_from_cards[card.color] += 1;

                                        let noble = state
                                            .nobles
                                            .iter()
                                            .enumerate()
                                            .filter(|&(_, ref noble)| {
                                                tokens_from_cards.can_buy(&noble.cost)
                                            })
                                            .map(|(i, _)| i as u8)
                                            .next();
                                        mov = Move::Buy {
                                            index: index,
                                            deck: deck,
                                            cost: cost,
                                            noble: noble,
                                        };
                                    } else {
                                        println!("can't afford");
                                        break 'outer;
                                    }
                                } else {
                                    println!("invalid card");
                                    break 'outer;
                                }
                            }
                            Some('r') => {
                                let cards;
                                let deck;
                                loop {
                                    let c = iter.next();
                                    match c {
                                        Some('1') => {
                                            cards = &state.cards1;
                                            deck = Deck::One;
                                            break;
                                        }
                                        Some('2') => {
                                            cards = &state.cards2;
                                            deck = Deck::Two;
                                            break;
                                        }
                                        Some('3') => {
                                            cards = &state.cards3;
                                            deck = Deck::Three;
                                            break;
                                        }
                                        Some(_) => continue,
                                        None => {
                                            println!("invalid command");
                                            break 'outer;
                                        }
                                    }
                                }

                                let index: u8;
                                loop {
                                    let c = iter.next();
                                    match c {
                                        Some('1') => {
                                            index = 0;
                                            break;
                                        }
                                        Some('2') => {
                                            index = 1;
                                            break;
                                        }
                                        Some('3') => {
                                            index = 2;
                                            break;
                                        }
                                        Some('4') => {
                                            index = 3;
                                            break;
                                        }
                                        Some(_) => continue,
                                        None => {
                                            println!("invalid command");
                                            break 'outer;
                                        }
                                    }
                                }

                                if (index as usize) < cards.len() {
                                    mov = Move::Reserve {
                                        index: index,
                                        deck: deck,
                                        drop: Tokens::empty(),
                                        joker: state.bank.joker > 0,
                                    };
                                } else {
                                    println!("invalid card");
                                    break 'outer;
                                }
                            }
                            Some('u') => {
                                let index: u8;
                                loop {
                                    let c = iter.next();
                                    match c {
                                        Some('1') => {
                                            index = 0;
                                            break;
                                        }
                                        Some('2') => {
                                            index = 1;
                                            break;
                                        }
                                        Some('3') => {
                                            index = 2;
                                            break;
                                        }
                                        Some(_) => continue,
                                        None => {
                                            println!("invalid command");
                                            break 'outer;
                                        }
                                    }
                                }

                                if let Some(ref card) = state.adversary.reserved.get(index as usize)
                                {
                                    if let Some(cost) = state.adversary.cost_for(card) {
                                        let mut tokens_from_cards =
                                            state.adversary.tokens_from_cards();
                                        tokens_from_cards[card.color] += 1;

                                        let noble = state
                                            .nobles
                                            .iter()
                                            .enumerate()
                                            .filter(|&(_, ref noble)| {
                                                tokens_from_cards.can_buy(&noble.cost)
                                            })
                                            .map(|(i, _)| i as u8)
                                            .next();
                                        mov = Move::BuyReserved {
                                            index: index,
                                            cost: cost,
                                            noble: noble,
                                        };
                                    } else {
                                        println!("can't afford");
                                        break 'outer;
                                    }
                                } else {
                                    println!("invalid card");
                                    break 'outer;
                                }
                            }
                            Some('p') => {
                                mov = Move::Pass;
                            }
                            Some(_) | None => continue,
                        }
                    }
                    Err(_) => {
                        panic!("couldn't read input");
                    }
                }

                let moves = state.generate_moves();
                if moves.iter().any(|m| m == &mov) {
                    println!("{:?}", &mov);
                    state.apply(&mov);
                    break;
                } else {
                    println!("Invalid move");
                }
            }
        }

        if state.cards1.len() < 4 {
            if let Some(card) = state.deck1.pop() {
                state.cards1.push(card);
            }
        }
        if state.cards2.len() < 4 {
            if let Some(card) = state.deck2.pop() {
                state.cards2.push(card);
            }
        }
        if state.cards3.len() < 4 {
            if let Some(card) = state.deck3.pop() {
                state.cards3.push(card);
            }
        }
        println!("");
    }
    println!("round: {}", round);
}

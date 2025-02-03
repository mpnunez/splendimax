extern crate rand;
extern crate splendimax;

use std::io;
// use std::time::Duration;
// use std::thread::sleep;
use rand::{thread_rng, Rng};
use splendimax::algo::alphabeta;
use splendimax::algo::state::Score;
use splendimax::algo::state::State as AlgoState;
use splendimax::state::State;

struct OppositeState<'a, S: AlgoState>(&'a mut S)
where
    S: 'a;

impl<'a, S: AlgoState> AlgoState for OppositeState<'a, S> {
    type Score = S::Score;
    type Move = S::Move;

    fn score(&self) -> Self::Score {
        self.0.score().neg()
    }

    fn generate_moves(&self) -> Vec<Self::Move> {
        self.0.generate_moves()
    }

    fn is_terminal(&self) -> bool {
        self.0.is_terminal()
    }

    fn apply(&mut self, mov: &Self::Move) {
        self.0.apply(mov);
    }

    fn undo(&mut self, mov: &Self::Move) {
        self.0.undo(mov);
    }
}

fn main() {
    //loop {
    let mut stdout = io::stdout();
    let mut state = State::new(2);
    let mut rng = thread_rng();
    let mut round = 0;
    loop {
        if state.is_terminal() {
            state.print(&mut stdout).unwrap();
            println!("round: {}", round);
            break;
        }
        let moves;
        if state.players_turn {
            moves = alphabeta(&mut state);
        } else {
            let mut opposite = OppositeState(&mut state);
            moves = alphabeta(&mut opposite);
            round += 1;
        }

        if let Some(mov) = rng.choose(&moves) {
            // state.print(&mut stdout);
            // println!("{:?}", mov);
            state.apply(&mov);
        } else {
            state.print(&mut stdout).unwrap();
            panic!("No moves");
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
        // sleep(Duration::from_secs(1));
    }
    //}
}

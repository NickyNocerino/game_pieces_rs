use crate::card::{Rank, Suit, Card};

pub struct Deck{
    pub size:usize,
    pub count_by_rank: [usize;13],
    pub count_by_suit: [usize;4],
    pub count_by_blackjack_value: [usize;10],
    pub cards:Vec<Card>,
}

impl Deck {
    pub fn new_empty() -> Self {
        Self{
            size:0,
            count_by_rank:[0usize;13],
            count_by_suit:[0usize;4],
            count_by_blackjack_value: [0usize;10],
            cards: Vec::<Card>::new(),
        }
    }
}
use rand::Rng;

use crate::card::{Rank, Suit, Card};

pub struct Deck{
    pub count:usize,
    pub count_by_rank: [usize;13],
    pub count_by_suit: [usize;4],
    pub count_by_blackjack_value: [usize;10],
    pub cards:Vec<Card>,
}

impl Deck {
    pub fn new_empty() -> Self {
        Self{
            count:0,
            count_by_rank:[0usize;13],
            count_by_suit:[0usize;4],
            count_by_blackjack_value: [0usize;10],
            cards: Vec::<Card>::new(),
        }
    }

    pub fn from_vec(cards:&Vec<Card>) -> Self {
        let mut count_by_rank = [0usize;13];
        let mut count_by_suit = [0usize;4];
        let mut count_by_blackjack_value = [0usize;10];
        
        for card in cards.clone() {
            count_by_rank[card.get_rank_index()] += 1;
            count_by_suit[card.get_suit_index()] += 1;
            count_by_blackjack_value[card.get_blackjack_value_index()] += 1;
        }

        Self{
            count:cards.len(),
            count_by_rank: count_by_rank,
            count_by_suit: count_by_suit,
            count_by_blackjack_value: count_by_blackjack_value,
            cards: cards.clone(),
        }
    }

    pub fn clone(&self) -> Self {
        let mut count_by_rank = [0usize;13];
        count_by_rank.clone_from_slice(&self.count_by_rank);
        let mut count_by_suit = [0usize;4];
        count_by_suit.clone_from_slice(&self.count_by_suit);
        let mut count_by_blackjack_value = [0usize;10];
        count_by_blackjack_value.clone_from_slice(&self.count_by_blackjack_value);
        Self{
            count: self.count.clone(),
            count_by_rank:count_by_rank,
            count_by_suit:count_by_suit,
            count_by_blackjack_value:count_by_blackjack_value,
            cards: self.cards.clone(),
        }
    }

    pub fn to_vec(&self) -> Vec<Card> {
        self.cards.clone()
    }

    pub fn draw(&self) -> (Card, Self) {
        //Draws randomly, does not preserve order for speed
        if self.count == 0 {
            panic!("Drawing from an empty deck")
        }
        let mut out_deck = self.cards.clone();
        let drawn_card = out_deck.swap_remove(rand::thread_rng().gen_range(0..out_deck.len()));
        let mut count_by_rank = [0usize;13];
        count_by_rank.clone_from_slice(&self.count_by_rank);
        count_by_rank[drawn_card.get_rank_index()] -= 1;
        let mut count_by_suit = [0usize;4];
        count_by_suit.clone_from_slice(&self.count_by_suit);
        count_by_suit[drawn_card.get_suit_index()] -= 1;
        let mut count_by_blackjack_value = [0usize;10];
        count_by_blackjack_value.clone_from_slice(&self.count_by_blackjack_value);
        count_by_blackjack_value[drawn_card.get_blackjack_value_index()] -= 1;
        let new_deck = Self{
            count:self.count-1,
            count_by_rank:count_by_rank,
            count_by_suit:count_by_suit,
            count_by_blackjack_value:count_by_blackjack_value,
            cards:out_deck
        };
        (drawn_card, new_deck)
    }

    pub fn draw_rank(&self, r:Rank) -> (Card, Self) {
        //Draws first matching found, does not preserve order for speed
        if self.count == 0 {
            panic!("Drawing from an empty deck")
        }
        let mut out_deck = self.cards.clone();
        for (i, card) in (self.cards.clone()).iter().enumerate(){
            if card.rank == r {
                let drawn_card = out_deck.swap_remove(i);
                return (drawn_card, Self::from_vec(&out_deck))
            }
        }
        panic!("drawning card of rank not in deck")
    }

    pub fn draw_suit(&self, s:Suit) -> (Card, Self) {
        //Draws first matching found, does not preserve order for speed
        if self.count == 0 {
            panic!("Drawing from an empty deck")
        }
        let mut out_deck = self.cards.clone();
        for (i, card) in (self.cards.clone()).iter().enumerate(){
            if card.suit == s {
                let drawn_card = out_deck.swap_remove(i);
                let mut count_by_rank = [0usize;13];
                count_by_rank.clone_from_slice(&self.count_by_rank);
                count_by_rank[drawn_card.get_rank_index()] -= 1;
                let mut count_by_suit = [0usize;4];
                count_by_suit.clone_from_slice(&self.count_by_suit);
                count_by_suit[drawn_card.get_suit_index()] -= 1;
                let mut count_by_blackjack_value = [0usize;10];
                count_by_blackjack_value.clone_from_slice(&self.count_by_blackjack_value);
                count_by_blackjack_value[drawn_card.get_blackjack_value_index()] -= 1;
                let new_deck = Self{
                    count:self.count-1,
                    count_by_rank:count_by_rank,
                    count_by_suit:count_by_suit,
                    count_by_blackjack_value:count_by_blackjack_value,
                    cards:out_deck
                };
                return (drawn_card, new_deck)
            }
        }
        panic!("drawning card of suit not in deck")
    }

    pub fn draw_blackjack_value_index(&self, n:usize) -> (Card, Self) {
        //Draws first matching found, does not preserve order for speed
        if self.count == 0 {
            panic!("Drawing from an empty deck")
        }
        let mut out_deck = self.cards.clone();
        for (i, card) in (self.cards.clone()).iter().enumerate(){
            if card.get_blackjack_value_index() == n {
                let drawn_card = out_deck.swap_remove(i);
                let mut count_by_rank = [0usize;13];
                count_by_rank.clone_from_slice(&self.count_by_rank);
                count_by_rank[drawn_card.get_rank_index()] -= 1;
                let mut count_by_suit = [0usize;4];
                count_by_suit.clone_from_slice(&self.count_by_suit);
                count_by_suit[drawn_card.get_suit_index()] -= 1;
                let mut count_by_blackjack_value = [0usize;10];
                count_by_blackjack_value.clone_from_slice(&self.count_by_blackjack_value);
                count_by_blackjack_value[drawn_card.get_blackjack_value_index()] -= 1;
                let new_deck = Self{
                    count:self.count-1,
                    count_by_rank:count_by_rank,
                    count_by_suit:count_by_suit,
                    count_by_blackjack_value:count_by_blackjack_value,
                    cards:out_deck
                };
                return (drawn_card, new_deck)
            }
        }
        panic!("drawning card of blackjack value not in deck")
    }

    pub fn draw_probs_by_rank(&self) -> [f64;13]
    {
        if self.count == 0 {
            panic!("Drawing odds from an empty deck")
        }
        let mut out = [0.0f64;13];
        for (index, count) in self.count_by_rank.to_vec().iter().enumerate() {
            out[index] = *count as f64 / self.count as f64;
        }
        out
    }

    pub fn draw_probs_by_suit(&self) -> [f64;4]
    {
        if self.count == 0 {
            panic!("Drawing odds from an empty deck")
        }
        let mut out = [0.0f64;4];
        for (index, count) in self.count_by_suit.to_vec().iter().enumerate() {
            out[index] = *count as f64 / self.count as f64;
        }
        out
    }

    pub fn draw_probs_by_blackjack_value(&self) -> [f64;10]
    {
        if self.count == 0 {
            panic!("Drawing odds from an empty deck")
        }
        let mut out = [0.0f64;10];
        for (index, count) in self.count_by_blackjack_value.to_vec().iter().enumerate() {
            out[index] = *count as f64 / self.count as f64;
        }
        out
    }
}
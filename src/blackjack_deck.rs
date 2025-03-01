use crate::card::{Card};

use rand::distributions::{WeightedIndex, Distribution};


pub struct BlackjackDeck{
    pub count:usize,
    pub count_by_blackjack_value: [usize;10],
}

impl BlackjackDeck {
    pub fn new_empty() -> Self {
        Self{
            count:0,
            count_by_blackjack_value: [0usize;10],
        }
    }
    pub fn from_vec(cards:&Vec<Card>) -> Self {
        
        let mut count_by_blackjack_value = [0usize;10];
        
        for card in cards.clone() {
            count_by_blackjack_value[card.get_blackjack_value_index()] += 1;
        }

        Self{
            count:cards.len(),
            count_by_blackjack_value: count_by_blackjack_value,
        }
    }

    pub fn clone(&self) -> Self {
        let mut count_by_blackjack_value = [0usize;10];
        count_by_blackjack_value.clone_from_slice(&self.count_by_blackjack_value);
        Self{
            count: self.count.clone(),
            count_by_blackjack_value:count_by_blackjack_value,
        }
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

    pub fn draw(&self) -> (usize, Self) {
        //Draws randomly, does not preserve order for speed
        if self.count == 0 {
            panic!("Drawing from an empty deck")
        }
        let draw_probs = self.draw_probs_by_blackjack_value();
        let weight_dist = WeightedIndex::new(draw_probs.iter().map(|x| x)).unwrap();
        let drawn_card_index = weight_dist.sample(&mut rand::thread_rng());
        let mut count_by_blackjack_value = [0usize;10];
        count_by_blackjack_value.clone_from_slice(&self.count_by_blackjack_value);
        count_by_blackjack_value[drawn_card_index] -= 1;
        let new_deck = Self{
            count:self.count-1,
            count_by_blackjack_value:count_by_blackjack_value,
        };
        (drawn_card_index, new_deck)
    }

    pub fn draw_blackjack_value_index(&self, drawn_card_index:usize) ->(usize, Self) {
        let mut count_by_blackjack_value = [0usize;10];
        count_by_blackjack_value.clone_from_slice(&self.count_by_blackjack_value);
        count_by_blackjack_value[drawn_card_index] -= 1;
        let new_deck = Self {
            count:self.count-1,
            count_by_blackjack_value:count_by_blackjack_value,
        };
        (drawn_card_index, new_deck)
    }

}
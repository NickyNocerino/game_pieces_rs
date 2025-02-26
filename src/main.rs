use game_pieces_rs::card::{Card};
use game_pieces_rs::deck::Deck;

fn main() {
    let mut rand_card = Card::new_random();
    println!("Rank: {:?}, Suit: {:?}", rand_card.rank, rand_card.suit);
    rand_card = Card::new_random();
    println!("Rank: {:?}, Suit: {:?}", rand_card.rank, rand_card.suit);
    rand_card = Card::new_random();
    println!("Rank: {:?}, Suit: {:?}", rand_card.rank, rand_card.suit);

    let mut deck = Deck::new_empty();
}

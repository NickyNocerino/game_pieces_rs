use game_pieces_rs::card::{Card};

fn main() {
    let mut rand_card = Card::new_random();
    println!("Rank: {:?}, Suit: {:?}", rand_card.rank, rand_card.suit);
    rand_card = Card::new_random();
    println!("Rank: {:?}, Suit: {:?}", rand_card.rank, rand_card.suit);
    rand_card = Card::new_random();
    println!("Rank: {:?}, Suit: {:?}", rand_card.rank, rand_card.suit);
}

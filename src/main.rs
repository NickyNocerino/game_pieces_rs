use game_pieces_rs::card::{Suit, Rank, Card};
use game_pieces_rs::deck::Deck;

fn main() {
    let mut rand_card = Card::new_random();
    println!("Rank: {:?}, Suit: {:?}", rand_card.rank, rand_card.suit);
    rand_card = Card::new_random();
    println!("Rank: {:?}, Suit: {:?}", rand_card.rank, rand_card.suit);
    rand_card = Card::new_random();
    println!("Rank: {:?}, Suit: {:?}", rand_card.rank, rand_card.suit);

    let mut deck = Deck::new_empty();
    let mut deck_list = Vec::<Card>::new();
    let suit_list = vec![Suit::Spades, Suit::Clubs, Suit::Diamonds, Suit::Hearts];
    let rank_list = vec![Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King];

    for rank in rank_list.iter(){
        for suit in suit_list.iter() {
            deck_list.push(Card::new(*rank, *suit));
        }
    }
    deck = Deck::from_vec(&deck_list);
    println!("deck suit odds: {:?}", deck.draw_probs_by_suit() );
    println!("deck rank odds: {:?}", deck.draw_probs_by_rank() );
    println!("deck blackjack odds: {:?}", deck.draw_probs_by_blackjack_value() );
}

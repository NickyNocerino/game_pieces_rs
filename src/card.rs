use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub enum Rank{
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone)]
pub enum Suit{
    Spades,
    Clubs,
    Diamonds,
    Hearts,
}

#[derive(Debug, Copy, Clone)]
pub struct Card{
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank:Rank, suit:Suit) -> Self {
        Self{rank:rank, suit:suit}
    }

    pub fn new_random() -> Self {
        let rank_num:usize = rand::thread_rng().gen_range(1..14);
        let mut r:Rank = Rank::Ace;
        match rank_num {
            1 => r = Rank::Ace,
            2 => r = Rank::Two,
            3 => r = Rank::Three,
            4 => r = Rank::Four,
            5 => r = Rank::Five,
            6 => r = Rank::Six,
            7 => r = Rank::Seven,
            8 => r = Rank::Eight,
            9 => r = Rank::Nine,
            10 => r = Rank::Ten,
            11 => r = Rank::Jack,
            12 => r = Rank::Queen,
            13 => r = Rank::King,
            0_usize | 14_usize.. => panic!("random number out of bounds"),
        }
        let suit_num:usize = rand::thread_rng().gen_range(1..5);
        let mut s:Suit = Suit::Spades;
        match suit_num {
            1 => s = Suit::Spades,
            2 => s = Suit::Clubs,
            3 => s = Suit::Diamonds,
            4 => s = Suit::Hearts,
            0_usize | 5_usize.. => panic!("random number out of bounds"),
        }
        Self{rank:r, suit:s}
    }
}
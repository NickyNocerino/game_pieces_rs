use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
        let r:Rank;
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
        let s:Suit;
        match suit_num {
            1 => s = Suit::Spades,
            2 => s = Suit::Clubs,
            3 => s = Suit::Diamonds,
            4 => s = Suit::Hearts,
            0_usize | 5_usize.. => panic!("random number out of bounds"),
        }
        Self{rank:r, suit:s}
    }
    
    pub fn get_suit_index(&self) -> usize {
        match self.suit {
            Suit::Spades => return 0,
            Suit::Clubs => return 1,
            Suit::Diamonds => return 2,
            Suit::Hearts => return 3,
        }
    }
    pub fn get_rank_index(&self) -> usize {
        match self.rank {
            Rank::Ace => return 0,
            Rank::Two => return 1,
            Rank::Three => return 2,
            Rank::Four => return 3,
            Rank::Five => return 4,
            Rank::Six => return 5,
            Rank::Seven => return 6,
            Rank::Eight => return 7,
            Rank::Nine => return 8,
            Rank::Ten => return 9,
            Rank::Jack => return 10,
            Rank::Queen => return 11,
            Rank::King => return 12,
        }
    }
    pub fn get_blackjack_value_index(&self) -> usize {
        match self.rank {
            Rank::Ace => return 0,
            Rank::Two => return 1,
            Rank::Three => return 2,
            Rank::Four => return 3,
            Rank::Five => return 4,
            Rank::Six => return 5,
            Rank::Seven => return 6,
            Rank::Eight => return 7,
            Rank::Nine => return 8,
            Rank::Ten => return 9,
            Rank::Jack => return 9,
            Rank::Queen => return 9,
            Rank::King => return 9,
        }
    }
}
use rand::*;

use super::*;

/// Shuffles the slice of cards
fn shuffle(cards: &mut [Card]) {
    let mut rng = thread_rng();
    // Knuth shuffle
    let num_cards = cards.len();
    for i in (1..num_cards - 1).rev() {
        let j = rng.gen_range(0, i);
        cards.swap(i, j);
    }
}

/// Sorts the slice by suit then rank (low to high)
fn sort_suit_ascending_rank(cards: &mut [Card]) {
    cards.sort_by(|a, b| a.cmp_suit_then_rank(b));
}

/// Sorts the slice by rank(high to low) and then suit
fn sort_suit_descending_rank(cards: &mut [Card]) {
    // Reverse sort (since default is low to high)
    cards.sort_by(|a, b| a.cmp_suit_then_desc_rank(b));
}

/// Sorts the slice by rank(high to low) and then suit
fn sort_descending_rank_suit(cards: &mut [Card]) {
    // Reverse sort (since default is low to high)
    cards.sort_by(|a, b| a.cmp_desc_rank_then_suit(b));
}

/// Returns cards of the specified rank
pub fn cards_of_rank(cards: &[Card], rank: Rank) -> Vec<Card> {
    cards.iter().filter(|c| c.rank == rank).cloned().collect()
}

/// Returns cards of the specified suit
pub fn cards_of_suit(cards: &[Card], suit: Suit) -> Vec<Card> {
    cards.iter().filter(|c| c.suit == suit).cloned().collect()
}

/// Certain actions are common to a deck and a hand of cards
pub trait Cards {
    fn cards(&self) -> &[Card];
    fn mut_cards(&mut self) -> &mut [Card];

    /// Shuffle the cards into a random order
    fn shuffle(&mut self) {
        shuffle(self.mut_cards());
    }

    /// Sort the cards by suit and then by rank (low to high)
    fn sort_suit_ascending_rank(&mut self) {
        sort_suit_ascending_rank(self.mut_cards());
    }

    /// Sorts the cards by suit and then by rank (high to low)
    fn sort_suit_descending_rank(&mut self) {
        sort_suit_descending_rank(self.mut_cards());
    }

    /// Sort the cards by rank (high to low) and then by suit
    fn sort_descending_rank_suit(&mut self) {
        sort_descending_rank_suit(self.mut_cards());
    }
}

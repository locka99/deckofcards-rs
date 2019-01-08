use rand::*;

use super::*;

/// Shuffles the slice of cards
fn shuffle(cards: &mut [Card]) {
    let mut rng = thread_rng();
    // Knuth shuffle
    let l = cards.len();
    for n in 0..l {
        let i = rng.gen_range(0, l - n);
        cards.swap(i, l - n - 1);
    }
}

#[test]
fn test_shuffle() {
    // This code is going create a bunch of decks and shuffle them. It will test that the cards at ends of the deck appear to be shuffled.
    let loop_count = 50;
    let mut top_matches = 0;
    let mut bottom_matches = 0;

    for _ in 0..loop_count {
        let mut d = Deck::new();
        // Get cards before shuffling
        let t1 = d.top_card().unwrap();
        let b1 = d.bottom_card().unwrap();
        // Shuffle
        d.shuffle();
        // Get end cards after shuffling
        let t2 = d.top_card().unwrap();
        let b2 = d.bottom_card().unwrap();
        // Increment if top and bottom appear to be unshuffled
        if t1 == t2 {
            top_matches += 1;
        }
        if b1 == b2 {
            bottom_matches += 1;
        }
    }

    println!(
        "top card matched {} times, bottom card matched {} times",
        top_matches, bottom_matches
    );

    // We expect shuffling of both top and bottom at least some of the iterations of the loop
    assert!(top_matches < loop_count);
    assert!(bottom_matches < loop_count);
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

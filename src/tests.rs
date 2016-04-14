use std::collections::HashSet;
use std::slice::Iter;
use super::*;

#[test]
fn value_to_str() {
    assert_eq!("Ace", Value::Ace.to_str());
    assert_eq!("Two", Value::Two.to_str());
    assert_eq!("Three", Value::Three.to_str());
    assert_eq!("Four", Value::Four.to_str());
    assert_eq!("Five", Value::Five.to_str());
    assert_eq!("Six", Value::Six.to_str());
    assert_eq!("Seven", Value::Seven.to_str());
    assert_eq!("Eight", Value::Eight.to_str());
    assert_eq!("Nine", Value::Nine.to_str());
    assert_eq!("Ten", Value::Ten.to_str());
    assert_eq!("Jack", Value::Jack.to_str());
    assert_eq!("Queen", Value::Queen.to_str());
    assert_eq!("King", Value::King.to_str());
}

#[test]
fn value_iter() {
    let mut i : Iter<'static, Value>  = Value::iterator();
    assert_eq!(*i.next().unwrap(), Value::Two);
    assert_eq!(*i.next().unwrap(), Value::Three);
    assert_eq!(*i.next().unwrap(), Value::Four);
    assert_eq!(*i.next().unwrap(), Value::Five);
    assert_eq!(*i.next().unwrap(), Value::Six);
    assert_eq!(*i.next().unwrap(), Value::Seven);
    assert_eq!(*i.next().unwrap(), Value::Eight);
    assert_eq!(*i.next().unwrap(), Value::Nine);
    assert_eq!(*i.next().unwrap(), Value::Ten);
    assert_eq!(*i.next().unwrap(), Value::Jack);
    assert_eq!(*i.next().unwrap(), Value::Queen);
    assert_eq!(*i.next().unwrap(), Value::King);
    assert_eq!(*i.next().unwrap(), Value::Ace);
}

#[test]
fn value_char() {
    assert_eq!(Value::from_char('A').unwrap(), Value::Ace);
    assert_eq!(Value::from_char('3').unwrap(), Value::Three);
    assert_eq!(Value::from_char('K').unwrap(), Value::King);
    assert!(Value::from_char(' ').is_err());
    assert!(Value::from_char('H').is_err());
    assert!(Value::from_char('x').is_err());
    assert_eq!(Value::Ace.to_char(), 'A');
    assert_eq!(Value::Three.to_char(), '3');
    assert_eq!(Value::King.to_char(), 'K');
}

#[test]
fn suit_to_str() {
    assert_eq!("Spades", Suit::Spades.to_str());
    assert_eq!("Hearts", Suit::Hearts.to_str());
    assert_eq!("Diamonds", Suit::Diamonds.to_str());
    assert_eq!("Clubs", Suit::Clubs.to_str());
}

#[test]
fn suit_iter() {
    let mut i : Iter<'static, Suit>  = Suit::iterator();
    assert_eq!(*i.next().unwrap(), Suit::Spades);
    assert_eq!(*i.next().unwrap(), Suit::Hearts);
    assert_eq!(*i.next().unwrap(), Suit::Diamonds);
    assert_eq!(*i.next().unwrap(), Suit::Clubs);
}

#[test]
fn suit_char() {
    assert_eq!(Suit::from_char('H').unwrap(), Suit::Hearts);
    assert_eq!(Suit::from_char('D').unwrap(), Suit::Diamonds);
    assert_eq!(Suit::from_char('S').unwrap(), Suit::Spades);
    assert_eq!(Suit::from_char('C').unwrap(), Suit::Clubs);
    assert!(Suit::from_char(' ').is_err());
    assert!(Suit::from_char('T').is_err());
    assert!(Suit::from_char('x').is_err());
    assert_eq!(Suit::Hearts.to_char(), 'H');
    assert_eq!(Suit::Diamonds.to_char(), 'D');
    assert_eq!(Suit::Spades.to_char(), 'S');
    assert_eq!(Suit::Clubs.to_char(), 'C');
}

#[test]
fn card_equality() {
    let card1 = Card::new(Suit::Hearts, Value::Ace);
    let card2 = Card::new(Suit::Hearts, Value::Ace);
    assert_eq!(card1, card1);
    assert_eq!(card1, card2);
    assert_eq!(card2, card1);
    let card3 = Card::new(Suit::Spades, Value::Ace);
    assert!(card1 != card3);
    let card4 = Card::new(Suit::Hearts, Value::Two);
    assert!(card1 != card4);
}

#[test]
fn card_from_str() {
    assert_eq!(Card::from_str("TC").unwrap(), Card::new(Suit::Clubs, Value::Ten));
    assert_eq!(Card::from_str("CT").unwrap(), Card::new(Suit::Clubs, Value::Ten));
    assert_eq!(Card::from_str("AD").unwrap(), Card::new(Suit::Diamonds, Value::Ace));
    assert_eq!(Card::from_str("1S").unwrap(), Card::new(Suit::Spades, Value::Ace));
    assert!(Card::from_str("ADC").is_err());
    assert!(Card::from_str("A").is_err());
    assert!(Card::from_str("C").is_err());
    assert!(Card::from_str("AA").is_err());
    assert!(Card::from_str("DD").is_err());
    assert!(Card::from_str("").is_err());
}

#[test]
fn card_to_str() {
    assert_eq!(Card::new(Suit::Clubs, Value::Ten).to_str(), "TC");
    assert_eq!(Card::new(Suit::Spades, Value::Queen).to_str(), "QS");
    assert_eq!(Card::new(Suit::Diamonds, Value::Ace).to_str(), "AD");
    assert_eq!(Card::new(Suit::Hearts, Value::Three).to_str(), "3H");
}

#[test]
fn card_all_cards() {
    let cards = Card::all_cards();
    assert_eq!(cards.len(), 52);
}

#[test]
fn deck_count() {
    let mut d = Deck::new();
    assert_eq!(d.dealt_count(), 0);
    assert_eq!(d.undealt_count(), 52);
    assert_eq!(d.count(), 52);

    let _ = d.deal_one();
    assert_eq!(d.dealt_count(), 1);
    assert_eq!(d.undealt_count(), 51);
    assert_eq!(d.count(), 52);

    let _ = d.deal(10);
    assert_eq!(d.dealt_count(), 11);
    assert_eq!(d.undealt_count(), 41);
    assert_eq!(d.count(), 52);

    let _ = d.deal(100);
    assert_eq!(d.dealt_count(), 52);
    assert_eq!(d.undealt_count(), 0);
    assert_eq!(d.count(), 52);
}

#[test]
fn deck_unique() {
    let mut set : HashSet<usize> = HashSet::new();
    let mut d = Deck::new();
    loop {
        let c = d.deal_one();
        if c.is_err() {
            break;
        }
        let card = c.unwrap();
        set.insert(card.ordinal());
    }
    assert_eq!(set.len(), d.count());
}

#[test]
fn deck_dealt_cards() {
    let mut d = Deck::new();
    let mut dealt = 0;
    loop {
        let c = d.deal_one();
        if c.is_err() {
            break;
        }
        dealt += 1;
    }
    assert_eq!(dealt, 52);
}

#[test]
fn deck_reset() {
    let c1 = Card::new(Suit::Hearts, Value::Ace);
    let c2 = Card::new(Suit::Clubs, Value::Two);
    let c3 = Card::new(Suit::Diamonds, Value::Three);
    let c4 = Card::new(Suit::Spades, Value::Four);
    let c5 = Card::new(Suit::Hearts, Value::Five);
    let c6 = Card::new(Suit::Clubs, Value::Six);
    let cards : [Card; 6] = [ c1, c2, c3, c4, c5, c6 ];
    let mut d = Deck::from_cards(&cards);
    assert_eq!(d.count(), 6);
    assert_eq!(d.deal_one().unwrap(), c6);
    assert_eq!(d.deal_one().unwrap(), c5);
    assert_eq!(d.deal_one().unwrap(), c4);
    assert_eq!(d.deal_one().unwrap(), c3);
    assert_eq!(d.deal_one().unwrap(), c2);
    assert_eq!(d.deal_one().unwrap(), c1);
    d.reset();
    // Partially deal
    assert_eq!(d.deal_one().unwrap(), c6);
    assert_eq!(d.deal_one().unwrap(), c5);
    assert_eq!(d.deal_one().unwrap(), c4);
    d.reset();
    assert_eq!(d.deal_one().unwrap(), c6);
    assert_eq!(d.deal_one().unwrap(), c5);
    assert_eq!(d.deal_one().unwrap(), c4);
    assert_eq!(d.deal_one().unwrap(), c3);
    assert_eq!(d.deal_one().unwrap(), c2);
    assert_eq!(d.deal_one().unwrap(), c1);
}

#[test]
fn deck_shuffle_same_cards() {
    let c1 = Card::new(Suit::Hearts, Value::Ace);
    let c2 = Card::new(Suit::Clubs, Value::Two);
    let c3 = Card::new(Suit::Diamonds, Value::Three);
    let c4 = Card::new(Suit::Spades, Value::Four);
    let c5 = Card::new(Suit::Hearts, Value::Five);
    let c6 = Card::new(Suit::Clubs, Value::Six);
    let cards : [Card; 6] = [ c1, c2, c3, c4, c5, c6 ];
    let mut d = Deck::from_cards(&cards);
    d.shuffle();
    let mut set : HashSet<Card> = HashSet::new();
    loop {
        let c = d.deal_one();
        if c.is_err() {
            break;
        }
        let card = c.unwrap();
        set.insert(card);
    }
    assert!(set.contains(&c1));
    assert!(set.contains(&c2));
    assert!(set.contains(&c3));
    assert!(set.contains(&c4));
    assert!(set.contains(&c5));
    assert!(set.contains(&c6));
}

#[test]
fn deck_shuffle_new_order() {
    let mut d = Deck::new();
    d.shuffle();
    loop {
        let c = d.deal_one();
        if c.is_err() {
            break;
        }
    }
}

// TODO value_sort
// TODO suit_sort
// TODO hand_sort_values_and_suits

#[test]
fn hand_sort() {
    // Create unordered hand
    let mut h = Hand::new();
    h.push(Card::new(Suit::Clubs, Value::Ten));
    h.push(Card::new(Suit::Clubs, Value::Two));
    h.push(Card::new(Suit::Hearts, Value::Ace));
    // Sort
    h.sort_descending_value_suit();
    let cards = h.cards();
    let sc1 = cards[0];
    let sc2 = cards[1];
    let sc3 = cards[2];
    println!("card 1 = {}", sc1.name());
    println!("card 2 = {}", sc2.name());
    println!("card 3 = {}", sc3.name());
    assert_eq!(sc1, Card::new(Suit::Hearts, Value::Ace));
    assert_eq!(sc2, Card::new(Suit::Clubs, Value::Ten));
    assert_eq!(sc3, Card::new(Suit::Clubs, Value::Two));
}

#[test]
fn hand_sort_shuffle_deck() {
    let mut deck = Deck::new();
    deck.shuffle();

    // Deal all the cards into the hand
    let mut hand = Hand::new();
    deck.deal_to_hand(&mut hand, 52);

    // Sort
    hand.sort_suit_ascending_value();
    // Compare to default deck
    let all_cards = Card::all_cards();
    let hand_cards = hand.cards();
    assert_eq!(all_cards.len(), hand_cards.len());

    println!("Debug - the actual sort order");
    {
        let mut i = hand_cards.iter();
        loop {
            let n = i.next();
            if n.is_none() {
                break;
            }
            println!("card x = {}", n.unwrap().name());
        }
    }

    println!("Check order is sorted");
    // Iterate
    let mut i_all = all_cards.iter();
    let mut i_hand = hand_cards.iter();

    loop {
        let n_all = i_all.next();
        let n_hand = i_hand.next();
        if n_all.is_none() {
            break;
        }
        let card_1 = n_all.unwrap();
        let card_2 = n_hand.unwrap();
        println!("card 1 = {}", card_1.name());
        println!("card 2 = {}", card_2.name());
        assert_eq!(card_1, card_2);
    }
}

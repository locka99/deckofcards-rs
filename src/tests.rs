use std::cmp::Ordering;
use std::collections::HashSet;
use std::slice::Iter;

use super::*;

#[test]
fn rank_to_str() {
    assert_eq!("Ace", Rank::Ace.to_str());
    assert_eq!("Two", Rank::Two.to_str());
    assert_eq!("Three", Rank::Three.to_str());
    assert_eq!("Four", Rank::Four.to_str());
    assert_eq!("Five", Rank::Five.to_str());
    assert_eq!("Six", Rank::Six.to_str());
    assert_eq!("Seven", Rank::Seven.to_str());
    assert_eq!("Eight", Rank::Eight.to_str());
    assert_eq!("Nine", Rank::Nine.to_str());
    assert_eq!("Ten", Rank::Ten.to_str());
    assert_eq!("Jack", Rank::Jack.to_str());
    assert_eq!("Queen", Rank::Queen.to_str());
    assert_eq!("King", Rank::King.to_str());
}

#[test]
fn rank_iter() {
    let mut i : Iter<'static, Rank>  = Rank::iterator();
    assert_eq!(*i.next().unwrap(), Rank::Two);
    assert_eq!(*i.next().unwrap(), Rank::Three);
    assert_eq!(*i.next().unwrap(), Rank::Four);
    assert_eq!(*i.next().unwrap(), Rank::Five);
    assert_eq!(*i.next().unwrap(), Rank::Six);
    assert_eq!(*i.next().unwrap(), Rank::Seven);
    assert_eq!(*i.next().unwrap(), Rank::Eight);
    assert_eq!(*i.next().unwrap(), Rank::Nine);
    assert_eq!(*i.next().unwrap(), Rank::Ten);
    assert_eq!(*i.next().unwrap(), Rank::Jack);
    assert_eq!(*i.next().unwrap(), Rank::Queen);
    assert_eq!(*i.next().unwrap(), Rank::King);
    assert_eq!(*i.next().unwrap(), Rank::Ace);
}

#[test]
fn rank_char() {
    assert_eq!(Rank::from_char('A').unwrap(), Rank::Ace);
    assert_eq!(Rank::from_char('3').unwrap(), Rank::Three);
    assert_eq!(Rank::from_char('K').unwrap(), Rank::King);
    assert!(Rank::from_char(' ').is_err());
    assert!(Rank::from_char('H').is_err());
    assert!(Rank::from_char('x').is_err());
    assert_eq!(Rank::Ace.to_char(), 'A');
    assert_eq!(Rank::Three.to_char(), '3');
    assert_eq!(Rank::King.to_char(), 'K');
}

#[test]
fn rank_cmp() {
    // Validate aces
    assert_eq!(Rank::Ace.cmp(&Rank::Two), Ordering::Greater);
    assert_eq!(Rank::Ace.cmp(&Rank::Ace), Ordering::Equal);
    assert_eq!(Rank::Two.cmp(&Rank::Ace), Ordering::Less);

    // Validate upwards
    assert_eq!(Rank::Three.cmp(&Rank::Two), Ordering::Greater);
    assert_eq!(Rank::Four.cmp(&Rank::Three), Ordering::Greater);
    assert_eq!(Rank::Five.cmp(&Rank::Four), Ordering::Greater);
    assert_eq!(Rank::Six.cmp(&Rank::Five), Ordering::Greater);
    assert_eq!(Rank::Seven.cmp(&Rank::Six), Ordering::Greater);
    assert_eq!(Rank::Eight.cmp(&Rank::Seven), Ordering::Greater);
    assert_eq!(Rank::Nine.cmp(&Rank::Eight), Ordering::Greater);
    assert_eq!(Rank::Ten.cmp(&Rank::Nine), Ordering::Greater);
    assert_eq!(Rank::Jack.cmp(&Rank::Ten), Ordering::Greater);
    assert_eq!(Rank::Queen.cmp(&Rank::Jack), Ordering::Greater);
    assert_eq!(Rank::King.cmp(&Rank::Queen), Ordering::Greater);

    // Validate downwards
    assert_eq!(Rank::Two.cmp(&Rank::Three), Ordering::Less);
    assert_eq!(Rank::Three.cmp(&Rank::Four), Ordering::Less);
    assert_eq!(Rank::Four.cmp(&Rank::Five), Ordering::Less);
    assert_eq!(Rank::Five.cmp(&Rank::Six), Ordering::Less);
    assert_eq!(Rank::Six.cmp(&Rank::Seven), Ordering::Less);
    assert_eq!(Rank::Seven.cmp(&Rank::Eight), Ordering::Less);
    assert_eq!(Rank::Eight.cmp(&Rank::Nine), Ordering::Less);
    assert_eq!(Rank::Nine.cmp(&Rank::Ten), Ordering::Less);
    assert_eq!(Rank::Ten.cmp(&Rank::Jack), Ordering::Less);
    assert_eq!(Rank::Jack.cmp(&Rank::Queen), Ordering::Less);
    assert_eq!(Rank::Queen.cmp(&Rank::King), Ordering::Less);

    // Compare with ace low
    assert_eq!(Rank::Ace.cmp_ace_low(&Rank::Two), Ordering::Less);
    assert_eq!(Rank::Ace.cmp_ace_low(&Rank::Ace), Ordering::Equal);
    assert_eq!(Rank::Two.cmp_ace_low(&Rank::Ace), Ordering::Greater);
    assert_eq!(Rank::King.cmp_ace_low(&Rank::Ace), Ordering::Greater);
    assert_eq!(Rank::Ace.cmp_ace_low(&Rank::King), Ordering::Less);
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
    let card1 = card!("AH");
    let card2 = card!("AH");
    assert_eq!(card1, card1);
    assert_eq!(card1, card2);
    assert_eq!(card2, card1);
    let card3 = card!("AS");
    assert!(card1 != card3);
    let card4 = card!("2H");
    assert!(card1 != card4);
}

#[test]
fn card_from_str() {
    assert_eq!(Card::from_str("TC").unwrap(), Card::new(Rank::Ten, Suit::Clubs));
    assert_eq!(Card::from_str("CT").unwrap(), Card::new(Rank::Ten, Suit::Clubs));
    assert_eq!(Card::from_str("AD").unwrap(), Card::new(Rank::Ace, Suit::Diamonds));
    assert_eq!(Card::from_str("1S").unwrap(), Card::new(Rank::Ace, Suit::Spades));
    assert!(Card::from_str("ADC").is_err());
    assert!(Card::from_str("A").is_err());
    assert!(Card::from_str("C").is_err());
    assert!(Card::from_str("AA").is_err());
    assert!(Card::from_str("DD").is_err());
    assert!(Card::from_str("").is_err());
}

#[test]
fn card_to_str() {
    assert_eq!(Card::new(Rank::Ten, Suit::Clubs).to_str(), "TC");
    assert_eq!(Card::new(Rank::Queen, Suit::Spades).to_str(), "QS");
    assert_eq!(Card::new(Rank::Ace, Suit::Diamonds).to_str(), "AD");
    assert_eq!(Card::new(Rank::Three, Suit::Hearts).to_str(), "3H");
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
    let c1 = card!("AH");
    let c2 = card!("2C");
    let c3 = card!("3D");
    let c4 = card!("4S");
    let c5 = card!("5H");
    let c6 = card!("6C");
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
    let c1 = card!("AH");
    let c2 = card!("2C");
    let c3 = card!("3D");
    let c4 = card!("4S");
    let c5 = card!("5H");
    let c6 = card!("6C");
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

// TODO rank_sort
// TODO suit_sort
// TODO hand_sort_ranks_and_suits

#[test]
fn hand_sort() {
    // Create unordered hand
    let mut h = hand!("TC", "2C", "AH");
    // Sort
    h.sort_descending_rank_suit();
    let cards = h.cards();
    let sc1 = cards[0];
    let sc2 = cards[1];
    let sc3 = cards[2];
    println!("card 1 = {}", sc1.name());
    println!("card 2 = {}", sc2.name());
    println!("card 3 = {}", sc3.name());
    assert_eq!(sc1, Card::new(Rank::Ace, Suit::Hearts));
    assert_eq!(sc2, Card::new(Rank::Ten, Suit::Clubs));
    assert_eq!(sc3, Card::new(Rank::Two, Suit::Clubs));
}

#[test]
fn hand_sort_shuffle_deck() {
    let mut deck = Deck::new();
    deck.shuffle();

    // Deal all the cards into the hand
    let mut hand = Hand::new();
    deck.deal_to_hand(&mut hand, 52);

    // Sort
    hand.sort_suit_ascending_rank();
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

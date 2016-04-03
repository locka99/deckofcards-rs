extern crate rand;

pub mod suit;
pub mod value;
pub mod card;
pub mod deck;

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::slice::Iter;

    use value::Value;
    use suit::Suit;
    use card::Card;
    use deck::Deck;

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
        assert_eq!(*i.next().unwrap(), Value::Ace);
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

        let _ = d.deal_many(10);
        assert_eq!(d.dealt_count(), 11);
        assert_eq!(d.undealt_count(), 41);
        assert_eq!(d.count(), 52);

        let _ = d.deal_many(100);
        assert_eq!(d.dealt_count(), 52);
        assert_eq!(d.undealt_count(), 0);
        assert_eq!(d.count(), 52);
    }

    #[test]
    fn deck_unique() {
        let mut set : HashSet<u8> = HashSet::new();
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
        let mut d = Deck::new_from(&cards);
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
        let mut d = Deck::new_from(&cards);
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
//          println!("card = {}", c.unwrap().name());
        }
    }
}

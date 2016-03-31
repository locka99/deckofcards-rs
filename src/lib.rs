extern crate rand;

pub mod cards;
pub mod deck;

#[cfg(test)]
mod test {
    use ::cards::Value;
    use ::cards::Suit;
    use ::cards::Card;

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
    fn suit_to_str() {
        assert_eq!("Hearts", Suit::Hearts.to_str());
        assert_eq!("Diamonds", Suit::Diamonds.to_str());
        assert_eq!("Clubs", Suit::Clubs.to_str());
        assert_eq!("Spades", Suit::Spades.to_str());
    }
	
	#[test]
	fn shuffle_deck() {
		
	}
}

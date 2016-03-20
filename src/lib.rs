pub mod cards;
pub mod deck;

#[cfg(test)]
mod test {
    use ::cards::Value;
    use ::cards::Suit;
    use ::cards::Card;

    #[test]
    fn it_works() {
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
}

use crate::suit::Suit;

#[derive(PartialEq, Debug)]
struct Card {
    number: u8,
    suit: Suit,
}

impl Card {
    fn new(number: u8, suit: Suit) -> Card {
        if number < 1 || 13 < number {
            panic!("トランプの数字は1以上13以下でなければなりません。")
        }

        Card { number, suit }
    }
}

#[test]
fn new_test1() {
    let card = Card::new(1, Suit::Club);

    assert_eq!(
        card,
        Card {
            number: 1,
            suit: Suit::Club,
        }
    );
}

#[test]
fn new_test2() {
    let card = Card::new(1, Suit::Club);

    assert_ne!(
        card,
        Card {
            number: 2,
            suit: Suit::Heart,
        }
    );
}

#[test]
#[should_panic]
fn new_test3() {
    Card::new(0, Suit::Club);
    Card::new(20, Suit::Club);
}

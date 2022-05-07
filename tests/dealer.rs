use texas_holdem::card::Card;
use texas_holdem::card::State::InDeck;
use texas_holdem::dealer::Dealer;
use texas_holdem::suit::Suit;

#[test]
fn it_works() {
    #[derive(Debug)]
    struct TestCase {
        name: String,
        expected: Dealer,
    }

    let table = [
        TestCase {
            expected: Dealer {
                deck: build_mock_deck(),
            },
            name: String::from("正常系1"),
        },
    ];

    for test_case in table {
        assert_eq!(
            Dealer::new(),
            test_case.expected,
            "Failed in the {:?}.",
            test_case,
        );
    }
}

fn build_mock_deck() -> Vec<Card> {
    vec![
        Card::new(1, Suit::Club, InDeck),
        Card::new(1, Suit::Heart, InDeck),
        Card::new(1, Suit::Diamond, InDeck),
        Card::new(1, Suit::Spade, InDeck),
        Card::new(2, Suit::Club, InDeck),
        Card::new(2, Suit::Heart, InDeck),
        Card::new(2, Suit::Diamond, InDeck),
        Card::new(2, Suit::Spade, InDeck),
        Card::new(3, Suit::Club, InDeck),
        Card::new(3, Suit::Heart, InDeck),
        Card::new(3, Suit::Diamond, InDeck),
        Card::new(3, Suit::Spade, InDeck),
        Card::new(4, Suit::Club, InDeck),
        Card::new(4, Suit::Heart, InDeck),
        Card::new(4, Suit::Diamond, InDeck),
        Card::new(4, Suit::Spade, InDeck),
        Card::new(5, Suit::Club, InDeck),
        Card::new(5, Suit::Heart, InDeck),
        Card::new(5, Suit::Diamond, InDeck),
        Card::new(5, Suit::Spade, InDeck),
        Card::new(6, Suit::Club, InDeck),
        Card::new(6, Suit::Heart, InDeck),
        Card::new(6, Suit::Diamond, InDeck),
        Card::new(6, Suit::Spade, InDeck),
        Card::new(7, Suit::Club, InDeck),
        Card::new(7, Suit::Heart, InDeck),
        Card::new(7, Suit::Diamond, InDeck),
        Card::new(7, Suit::Spade, InDeck),
        Card::new(8, Suit::Club, InDeck),
        Card::new(8, Suit::Heart, InDeck),
        Card::new(8, Suit::Diamond, InDeck),
        Card::new(8, Suit::Spade, InDeck),
        Card::new(9, Suit::Club, InDeck),
        Card::new(9, Suit::Heart, InDeck),
        Card::new(9, Suit::Diamond, InDeck),
        Card::new(9, Suit::Spade, InDeck),
        Card::new(10, Suit::Club, InDeck),
        Card::new(10, Suit::Heart, InDeck),
        Card::new(10, Suit::Diamond, InDeck),
        Card::new(10, Suit::Spade, InDeck),
        Card::new(11, Suit::Club, InDeck),
        Card::new(11, Suit::Heart, InDeck),
        Card::new(11, Suit::Diamond, InDeck),
        Card::new(11, Suit::Spade, InDeck),
        Card::new(12, Suit::Club, InDeck),
        Card::new(12, Suit::Heart, InDeck),
        Card::new(12, Suit::Diamond, InDeck),
        Card::new(12, Suit::Spade, InDeck),
        Card::new(13, Suit::Club, InDeck),
        Card::new(13, Suit::Heart, InDeck),
        Card::new(13, Suit::Diamond, InDeck),
        Card::new(13, Suit::Spade, InDeck),
    ]
}
use crate::card::Card;

#[derive(PartialEq, Debug)]
pub struct Board {
    pub cards: Vec<Card>,
}

impl Board {
    pub fn new(cards: Vec<Card>) -> Board {
        if cards.len() > 5 {
            panic!("No more than 6 cards can be lined up on the board");
        }

        Board {
            cards
        }
    }
}

#[test]
fn it_works() {
    use crate::suit::Suit;
    use crate::card::State;

    #[derive(Debug)]
    struct TestCase {
        args: Vec<Card>,
        expected: Board,
        name: String,
    }

    let table = [
        TestCase {
            args: vec![Card::new(1, Suit::Club, State::InHand)],
            expected: Board {
                cards: vec![Card::new(1, Suit::Club, State::InHand)]
            },
            name: String::from("正常系1"),
        },
    ];

    for test_case in table {
        assert_eq!(
            Board::new(test_case.args.clone()),
            test_case.expected,
            "Failed in the {:?}.",
            test_case,
        );
    }
}

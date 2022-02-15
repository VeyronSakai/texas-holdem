use texas_holdem::board::Board;
use texas_holdem::card::Card;
use texas_holdem::card::State::{self, InDeck, InHand};
use texas_holdem::suit::Suit;

// #[test]
// fn it_works() {
//     #[derive(Debug)]
//     struct TestCase {
//         args: Vec<Card>,
//         expected: Board,
//         name: String,
//     }
//
//     let table = [
//         TestCase {
//             args: vec![Card::new(1, Suit::Club, State::InHand)],
//             expected: Board {
//                 cards: vec![Card::new(1, Suit::Club, State::InHand)]
//             },
//             name: String::from("正常系1"),
//         },
//     ];
//
//     for test_case in table {
//         assert_eq!(
//             Board::new(test_case.args.clone()),
//             test_case.expected,
//             "Failed in the {:?}.",
//             test_case,
//         );
//     }
// }

test_macro::test_assert_eq!(
    eq_case1,
    Board::new(vec![Card::new(1, Suit::Club, State::InHand)]) =>
    Board {cards: vec![Card::new(1, Suit::Club, State::InHand)]}
);

test_macro::test_assert_eq!(
    eq_case2,
    Board::new(vec![Card::new(10, Suit::Heart, State::OnBoard)]) =>
    Board {cards: vec![Card::new(10, Suit::Heart, State::OnBoard)]}
);

test_macro::test_assert_ne!(
    ne_case1,
    Board::new(vec![Card::new(1, Suit::Club, State::InHand)]) =>
    Board {cards: vec![Card::new(2, Suit::Club, State::InHand)]}
);

test_macro::test_assert_ne!(
    ne_case2,
    Board::new(vec![Card::new(1, Suit::Club, State::InHand)]) =>
    Board {cards: vec![Card::new(1, Suit::Heart, State::InHand)]}
);

test_macro::test_assert_ne!(
    ne_case3,
    Board::new(vec![Card::new(1, Suit::Club, State::InHand)]) =>
    Board {cards: vec![Card::new(1, Suit::Club, State::OnBoard)]}
);
use crate::domain::card::{Card, State};
use crate::domain::player::{Player, HAND_NUM};
use crate::domain::player_factory::PlayerFactory;
use crate::domain::suit::Suit;

struct PlayerFactoryImpl {}

impl PlayerFactoryImpl {
    fn new() -> Self {
        PlayerFactoryImpl {}
    }
}

impl PlayerFactory for PlayerFactoryImpl {
    fn create(&self, hands: [Card; HAND_NUM], chip: i32) -> Player {
        Player::new(hands, chip)
    }
}

#[test]
fn it_works() {
    let factory = PlayerFactoryImpl::new();
    let player = factory.create(
        [Card::new(1, Suit::Club, State::InHand), Card::new(1, Suit::Club, State::InHand)],
        10,
    );

    assert_eq!(
        player,
        Player {
            cards: [
                Card::new(1, Suit::Club, State::InHand),
                Card::new(1, Suit::Club, State::InHand)
            ],
            chip: 10,
        }
    );
}

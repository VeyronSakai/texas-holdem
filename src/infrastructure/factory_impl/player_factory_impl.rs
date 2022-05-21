use uuid::Uuid;
use crate::domain::card::{Card, State};
use crate::domain::player::{Player, HAND_NUM};
use crate::domain::player::player_id::PlayerId;
use crate::domain::player::player_factory::PlayerFactory;
use crate::domain::suit::Suit;

struct PlayerFactoryImpl {}

impl PlayerFactoryImpl {
    fn new() -> Self {
        PlayerFactoryImpl {}
    }
}

impl PlayerFactory for PlayerFactoryImpl {
    fn create(&self, hands: [Card; HAND_NUM], chip: i32) -> Player {
        let player_id = PlayerId::new(Uuid::new_v4());
        Player::new(player_id, hands, chip)
    }
}


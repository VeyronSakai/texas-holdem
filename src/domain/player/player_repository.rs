use crate::domain::player::player_id::PlayerId;
use crate::domain::player::Player;

pub trait PlayerRepository {
    fn find(&self, id: PlayerId) -> &Player;
    fn save(&mut self, player: Player);
}

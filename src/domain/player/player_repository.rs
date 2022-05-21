use crate::domain::player::Player;

pub trait PlayerRepository {
    fn find(id: u32) -> Player;
    fn save(player: Player);
}

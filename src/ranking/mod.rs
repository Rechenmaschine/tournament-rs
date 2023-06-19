use crate::player::PlayerId;

mod bradley_terry;

pub trait RankingPolicy {
    /// Returns the current ranking of players based on the scores table.
    fn rank_players(&mut self) -> Vec<PlayerId>;
}

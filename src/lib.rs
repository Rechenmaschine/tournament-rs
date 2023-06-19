#![allow(dead_code)]

mod executor;
mod player_repository;
mod ranking;
mod scheduling;
mod tournaments;

use anyhow::Error;
use crate::player_repository::Player;

type PlayerId = usize;

pub trait MatchResult {
    fn player1_win(&self) -> bool;
    fn player2_win(&self) -> bool;
    fn is_draw(&self) -> bool;
}

pub trait Match: Sized {
    /// All agents must be of the same type. If this is not the case,
    /// then you can box a trait object.
    type Agent: Clone + Sized + Send + Sync;

    /// The outcome of a match.
    type MatchResult: MatchResult + Send;

    /// Create a new match between two agents.
    ///
    /// Agents are cloned before each match, and can therefore be consumed by this function.
    /// By convention, the first player should always start the game.
    fn new(player1: Player<Self::Agent>, player2: Player<Self::Agent>) -> Self;

    /// Play the match and return the result.
    /// If the match cannot be played, return an error.
    fn playout(&mut self) -> Result<Self::MatchResult, Error>;
}

pub trait ScoringPolicy<M: Match> {
    /// Updates the scores table based on the result of a match.
    fn report(&mut self, match_result: M::MatchResult);
}

pub trait RankingPolicy {
    /// Returns the current ranking of players based on the scores table.
    fn rank_players(&mut self) -> Vec<PlayerId>;
}

pub trait Tournament<M, P, S, R> {
    fn start(&mut self);
}

//tests
#[cfg(test)]
mod tests {
    use super::*;

    use tournaments::Tournament;
    use crate::player_repository::PlayerRepository;
    use crate::scheduling::RoundRobbinScheduler;

    #[test]
    fn main(){

        let players = PlayerRepository::new(vec![]);

        let tourament = Tournament::new(
            RoundRobbinScheduler::new(players),
            (),
            (),
            ()
        ).with_threads(12);



    }

}
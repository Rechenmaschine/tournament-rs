use std::fmt::Debug;
use anyhow::Error;
use crate::player::{Player, PlayerId};

pub trait MatchResult {
    fn winner(&self) -> Option<PlayerId>;
    fn loser(&self) -> Option<PlayerId>;
    fn is_draw(&self) -> Option<(PlayerId, PlayerId)>;
}

pub trait Match: Sized {
    /// All agents must be of the same type. If this is not the case,
    /// then you can box a trait object.
    type Agent: Clone + Sized + Send + Sync;

    /// The outcome of a match.
    type MatchResult: MatchResult + Send + Debug;

    /// Create a new match between two agents.
    ///
    /// Agents are cloned before each match, and can therefore be consumed by this function.
    /// By convention, the first player should always start the game.
    fn new(player1: Player<Self>, player2: Player<Self>) -> Self;

    /// Play the match and return the result.
    /// If the match cannot be played, return an error.
    fn playout(&mut self) -> Result<Self::MatchResult, Error>;
}

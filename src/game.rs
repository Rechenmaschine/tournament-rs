use crate::player::{Player, PlayerData};
use anyhow::Error;
use std::fmt::Debug;

pub trait MatchResult {
    fn outcome(&self) -> Outcome;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    WinP1,
    WinP2,
    Draw,
}

#[derive(Debug, Clone)]
pub enum LabelledOutcome {
    Win {
        winner: PlayerData,
        loser: PlayerData,
    },
    Draw(PlayerData, PlayerData),
}

impl LabelledOutcome {
    pub fn new(outcome: Outcome, player1: PlayerData, player2: PlayerData) -> Self {
        match outcome {
            Outcome::WinP1 => LabelledOutcome::Win {
                winner: player1,
                loser: player2,
            },
            Outcome::WinP2 => LabelledOutcome::Win {
                winner: player2,
                loser: player1,
            },
            Outcome::Draw => LabelledOutcome::Draw(player1, player2),
        }
    }
}

pub trait Match: Sized {
    /// All agents must be of the same type. If this is not the case,
    /// then you can box a trait object.
    type Agent: Send;

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

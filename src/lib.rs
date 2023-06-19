#![allow(dead_code)]

pub mod game;
pub mod player;
pub mod ranking;
pub mod scheduling;
pub mod scoring;
pub mod tournaments;

pub mod prelude {
    pub use crate::game::{Match, MatchResult};
    pub use crate::player::{Player, PlayerId, PlayerRepository};
    pub use crate::ranking::RankingPolicy;
    pub use crate::scheduling::Scheduler;
    pub use crate::scoring::ScoringSystem;
    pub use crate::tournaments::Tournament;
}
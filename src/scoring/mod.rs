use crate::game::{LabelledOutcome, Match};
use crate::player::PlayerId;
use std::collections::HashMap;

pub trait ScoringSystem<M: Match> {
    /// Updates the scores table based on the result of a match.
    fn report(&mut self, outcome: &LabelledOutcome, match_result: &M::MatchResult);
}

pub struct DefaultScoring {
    player_scores: HashMap<PlayerId, i32>,
}

impl DefaultScoring {
    pub fn new(players: Vec<PlayerId>) -> Self {
        let player_scores = players.into_iter().map(|player| (player, 0)).collect();

        DefaultScoring { player_scores }
    }
}

impl<M: Match> ScoringSystem<M> for DefaultScoring {
    fn report(&mut self, outcome: &LabelledOutcome, _: &M::MatchResult) {
        match outcome {
            LabelledOutcome::Win { winner, .. } => {
                self.player_scores
                    .entry(winner.id())
                    .and_modify(|score| *score += 1);
            }
            LabelledOutcome::Draw(p1, p2) => {
                self.player_scores
                    .entry(p1.id())
                    .and_modify(|score| *score += 1);
                self.player_scores
                    .entry(p2.id())
                    .and_modify(|score| *score += 1);
            }
        }
    }
}

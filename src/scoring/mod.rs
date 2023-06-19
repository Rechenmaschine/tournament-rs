use crate::{Match, MatchResult, PlayerId, ScoringSystem};
use std::collections::HashMap;

pub struct DefaultScoring
{
    player_scores: HashMap<PlayerId, i32>,
}

impl DefaultScoring
where {
    pub fn new(players: Vec<PlayerId>) -> Self {
        let player_scores = players.into_iter().map(|player| (player, 0)).collect();

        DefaultScoring {
            player_scores,
        }
    }
}

impl<M: Match> ScoringSystem<M> for DefaultScoring {
    fn report(&mut self, match_result: M::MatchResult) {
        if let Some((p1, p2)) = match_result.is_draw() {
            // If draw, then both players get 1 point
            self.player_scores.entry(p1).and_modify(|score| *score += 1);
            self.player_scores.entry(p2).and_modify(|score| *score += 1);
        } else {
            // winner gets 2 points, loser gets 0
            self.player_scores
                .entry(match_result.winner().unwrap())
                .and_modify(|score| *score += 2);
        }
    }
}

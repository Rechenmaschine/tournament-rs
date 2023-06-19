use crate::{Match, MatchResult, PlayerId, ScoringSystem};
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct DefaultScoring<M>
where
    M: Match,
{
    player_scores: HashMap<PlayerId, i32>,
    _marker: PhantomData<M>,
}

impl<M> DefaultScoring<M>
where
    M: Match,
{
    pub fn new(players: Vec<PlayerId>) -> Self {
        let player_scores = players.into_iter().map(|player| (player, 0)).collect();

        DefaultScoring {
            player_scores,
            _marker: PhantomData,
        }
    }
}

impl<M: Match> ScoringSystem<M> for DefaultScoring<M> {
    fn report(&mut self, match_result: M::MatchResult) {

    }
}

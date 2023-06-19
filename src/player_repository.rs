// A Player repository maps PlayerIDs to their respective Agent.

use crate::{Match, PlayerId};
use anyhow::{anyhow, Error};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Player<A: Clone> {
    pub(crate) id: PlayerId,
    pub(crate) agent: A,
}

impl<A: Clone> PartialEq for Player<A> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<A: Clone> Hash for Player<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub struct PlayerRepository<M: Match> {
    players: HashMap<PlayerId, Player<M::Agent>>,
}

impl<M: Match> PlayerRepository<M> {
    // TODO: use a better id generation
    //  pass vec<player> instead of vec<agent>
    pub fn new(players: Vec<M::Agent>) -> Self {
        let mut players_map = HashMap::new();

        for (id, agent) in players.into_iter().enumerate() {
            let player = Player {
                id: id as PlayerId,
                agent,
            };
            players_map.insert(id, player);
        }

        PlayerRepository {
            players: players_map,
        }
    }

    pub fn ids(&self) -> Vec<PlayerId> {
        self.players.keys().cloned().collect()
    }

    pub fn get(&self, id: PlayerId) -> Result<Player<M::Agent>, Error> {
        self.players
            .get(&id)
            .cloned()
            .ok_or_else(|| anyhow!("Player with id {} not found", id))
    }
}

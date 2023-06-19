// A Player repository maps PlayerIDs to their respective Agent.

use crate::{Match, PlayerId};
use anyhow::{anyhow, Error};
use std::collections::HashMap;

pub struct Player<M: Match> {
    pub(crate) agent: M::Agent,
    pub(crate) name: &'static str,
}

impl<M: Match> Clone for Player<M> {
    fn clone(&self) -> Self {
        Player {
            agent: self.agent.clone(),
            name: self.name,
        }
    }
}

impl<M: Match> Player<M> {

    fn new(agent: M::Agent) -> Self {
        Player {
            agent,
            name: std::any::type_name::<M::Agent>()
        }
    }

    fn with_name(self, name: &'static str) -> Self {
        Player { name, ..self }
    }
}

pub struct PlayerRepository<M: Match> {
    players: HashMap<PlayerId, Player<M>>,
}

// TODO improve ID generation
impl<M: Match> PlayerRepository<M> {
    /// Creates a new PlayerRepository with the given players.
    pub fn new_with_players(players: Vec<Player<M>>) -> Self {
        let mut players_map = HashMap::new();

        let mut id = 1;
        for player in players.into_iter() {
            players_map.insert(id, player);
            id += 1;
        }

        PlayerRepository {
            players: players_map,
        }
    }

    /// Creates an empty PlayerRepository.
    pub fn new() -> Self {
        PlayerRepository {
            players: HashMap::new(),
        }
    }

    pub fn add(&mut self, player: Player<M>) {
        let id = self.players.len() as PlayerId + 1;
        self.players.insert(id, player);
    }

    pub fn players(&self) -> Vec<Player<M>> {
        self.players.values()
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn ids(&self) -> Vec<PlayerId> {
        self.players.keys().cloned().collect()
    }

    pub fn get(&self, id: PlayerId) -> Result<Player<M>, Error> {
        self.players
            .get(&id)
            .cloned()
            .ok_or_else(|| anyhow!("Player with id {} not found", id))
    }
}

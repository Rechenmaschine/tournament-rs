use crate::game::Match;
use anyhow::{anyhow, Error};
use std::collections::HashMap;

pub type PlayerId = usize;

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    name: &'static str,
}

pub struct PlayerGen<M: Match> {
    generator: fn() -> M::Agent,
    info: PlayerInfo,
}

// impl clone for PlayerGen
impl<M: Match> Clone for PlayerGen<M> {
    fn clone(&self) -> Self {
        PlayerGen {
            generator: self.generator,
            info: self.info.clone(),
        }
    }
}

// impl PlayerGenerator<M: Match>
impl<M: Match> PlayerGen<M> {
    pub fn new(agent_generator: fn() -> M::Agent) -> Self {
        PlayerGen {
            generator: agent_generator,
            info: PlayerInfo {
                name: std::any::type_name::<M::Agent>(),
            },
        }
    }

    pub fn with_name(self, name: &'static str) -> Self {
        PlayerGen {
            info: PlayerInfo { name },
            ..self
        }
    }

    pub fn generate_agent(&self) -> M::Agent {
        (self.generator)()
    }
}

pub struct Player<M: Match> {
    inner: M::Agent,
    info: PlayerInfo,
    id: PlayerId,
}

impl<M: Match> Player<M> {
    pub fn info(&self) -> &PlayerInfo {
        &self.info
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn name(&self) -> &'static str {
        self.info.name
    }

    pub fn unpack(self) -> M::Agent {
        self.inner
    }

    pub fn data(&self) -> PlayerData {
        PlayerData {
            info: self.info.clone(),
            id: self.id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerData {
    info: PlayerInfo,
    id: PlayerId,
}

impl PlayerData {
    pub fn info(&self) -> &PlayerInfo {
        &self.info
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn name(&self) -> &'static str {
        self.info.name
    }
}

#[derive(Clone)]
pub struct PlayerRepository<M: Match> {
    players: HashMap<PlayerId, PlayerGen<M>>,
}

impl<M: Match> PlayerRepository<M> {
    pub fn new() -> Self {
        PlayerRepository {
            players: HashMap::new(),
        }
    }

    pub fn ids(&self) -> Vec<PlayerId> {
        let mut v = self.players.keys().cloned().collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    pub fn add_player(&mut self, player: PlayerGen<M>) {
        let id = self.players.len(); // TODO improve
        self.players.insert(id, player);
    }

    pub fn get_raw(&self, id: PlayerId) -> Result<PlayerGen<M>, Error> {
        self.players
            .get(&id)
            .map(|player| player.clone())
            .ok_or_else(|| anyhow!("Player with id {} not found", id))
    }

    pub fn get_instance(&self, id: PlayerId) -> Result<Player<M>, Error> {
        self.players
            .get(&id)
            .map(|player| Player {
                inner: player.generate_agent(),
                info: player.info.clone(),
                id,
            })
            .ok_or_else(|| anyhow!("Player with id {} not found", id))
    }
}

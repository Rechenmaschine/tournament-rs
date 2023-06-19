/*------------------------------------------------------------------------------------------------*/
/*--------------------------------- Round Robbin Scheduler ---------------------------------------*/
/*------------------------------------------------------------------------------------------------*/

use crate::scheduling::{PlayerBalancing, Scheduler};
use crate::PlayerId;
use anyhow::{anyhow, Error};
use std::collections::VecDeque;

pub struct RoundRobbinScheduler {
    players: Vec<Option<PlayerId>>,
    pairs: Option<Vec<(PlayerId, PlayerId)>>,
    r: usize,
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

impl RoundRobbinScheduler {
    pub fn new(players: Vec<PlayerId>) -> Self {
        let mut players = players.into_iter().map(|p| Some(p)).collect::<Vec<_>>();

        // If the number of players is odd, add a bye player.
        // For scheduling convenience, the bye player will be fixed in place.
        if players.len() % 2 == 1 {
            players.insert(0, None);
        }

        RoundRobbinScheduler {
            players,
            r: 1,
            pairs: None,
        }
    }

    /// Set the rotation value to get a different round robin tournament
    /// The value must be coprime to `(#players-1)` including the bye player.
    /// The default value is 1.
    ///
    /// Returns an error if the value is not coprime to `(#players-1)` or
    /// the value is out of bounds.
    pub fn with_r(self, r: usize) -> Result<Self, Error> {
        if r == 0 || r >= self.players.len() {
            return Err(anyhow!("r must be in range [1, n-1]"));
        }

        if gcd(r, self.players.len() - 1) == 1 {
            Ok(RoundRobbinScheduler { r, ..self })
        } else {
            Err(anyhow!(
                "r must be coprime to (n-1), including the bye player"
            ))
        }
    }
}

impl Scheduler for RoundRobbinScheduler {
    /// Generates all distinct pairs for the round robin tournament.
    /// The number of rounds is n-1, where n is the number of players.
    fn init(&mut self) {
        if self.pairs.is_some() {
            eprintln!("Warning: RoundRobbinScheduler init method has already been called once");
        }

        let mut pairs = Vec::new();

        // Isolate the first player, and fix it in place.
        let fixed_player = self.players[0];
        let mut rest = self.players[1..].iter().cloned().collect::<VecDeque<_>>();

        let offset = self.players.len() / 2;
        let rounds = self.players.len() - 1;

        for round in 0..rounds {
            let p1 = fixed_player;
            let p2 = rest[0].unwrap();

            if p1.is_some() {
                // else p1 is bye player and we skip this pair
                // Alternate for fairness to each side
                if round % 2 == 0 {
                    pairs.push((p1.unwrap(), p2));
                } else {
                    pairs.push((p2, p1.unwrap()));
                }
            }

            for i in 1..offset {
                let p1 = rest[i].unwrap();
                let p2 = rest[rest.len() - i].unwrap();

                pairs.push((p1, p2));
            }
            // rotate the players, while fixing the first player
            rest.rotate_right(self.r);
        }
        self.pairs = Some(pairs);
    }

    fn get(&mut self) -> Option<(PlayerId, PlayerId)> {
        if self.pairs.is_none() {
            // TODO remove this
            eprintln!("Warning: RoundRobbinScheduler get method called before init method");
            self.init();
        }

        if self.pairs.as_ref().unwrap().is_empty() {
            None
        } else {
            Some(self.pairs.as_mut().unwrap().pop().unwrap())
        }
    }
}

impl Iterator for RoundRobbinScheduler {
    type Item = (PlayerId, PlayerId);

    fn next(&mut self) -> Option<Self::Item> {
        self.get()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.players.len();
        let n = n * (n - 1) / 2;
        (n, Some(n))
    }
}

// characteristics of round robbin
impl PlayerBalancing for RoundRobbinScheduler {}

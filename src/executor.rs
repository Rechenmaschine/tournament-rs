use crate::player_repository::PlayerRepository;
use crate::{Match, PlayerId};
use anyhow::{anyhow, Error};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};

pub struct Executor<M: Match> {
    pairing_policy: spmc::Receiver<Option<(PlayerId, PlayerId)>>,
    scoring_policy: mpsc::Sender<M::MatchResult>,
    player_repository: Arc<PlayerRepository<M>>,
    stop_signal: Arc<AtomicBool>,
}

/// An executor is in charge of running matches. It receives pairs of players from
/// the scheduler and reports back the results to the scoring policy.
impl<M> Executor<M>
where
    M: Match,
{
    /// Starts the executor. This function will keep running until the
    /// no more pairs are supplied by the scheduler or the stop signal
    /// is received.
    ///
    /// Returns the termination reason or an error if something went wrong.
    pub fn run(&self) -> Result<TerminationCondition, Error> {
        loop {
            // Check if the stop signal has been received
            if self.stop_signal.load(Ordering::Relaxed) {
                return Ok(TerminationCondition::StopSignalReceived);
            }

            // Get the next pair. If None is returned, the round (or tournament)
            // is over and the executor can stop.
            let (p1, p2) = match self.pairing_policy.recv() {
                Ok(Some(pair)) => pair,
                Ok(None) => return Ok(TerminationCondition::NoMorePairings),
                Err(_) => return Err(anyhow!("Error receiving match pair from scheduler")),
            };

            // Get the players from the player repository
            let p1 = self.player_repository.get(p1)?;
            let p2 = self.player_repository.get(p2)?;

            // Create the match from the pair
            let mut match_ = M::new(p1, p2);

            // Play the match
            let result = match_.playout()?;

            // Send the result to be processed
            if let Err(_) = self.scoring_policy.send(result) {
                return Err(anyhow!("Error sending match result to scoring policy"));
            }
        }
    }

    /// Creates a new executor.
    pub fn new(
        pairing_policy: spmc::Receiver<Option<(PlayerId, PlayerId)>>,
        scoring_policy: mpsc::Sender<M::MatchResult>,
        player_repository: Arc<PlayerRepository<M>>,
        stop_signal: Arc<AtomicBool>,
    ) -> Self {
        Executor {
            pairing_policy,
            scoring_policy,
            player_repository,
            stop_signal,
        }
    }
}

#[derive(Debug)]
pub enum TerminationCondition {
    StopSignalReceived,
    NoMorePairings,
    Custom(&'static str),
}

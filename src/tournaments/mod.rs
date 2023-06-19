use crate::executor::{Executor, TerminationCondition};
use crate::player_repository::PlayerRepository;
use crate::scheduling::Scheduler;
use crate::{Match, ScoringSystem};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::thread::JoinHandle;
use anyhow::Error;

pub struct Tournament<M, P, S, /*C*/>
    where
        M: Match,
        P: Scheduler,
        S: ScoringSystem<M>,
        // C: RankingPolicy,
{
    scheduler: P,
    scoring_policy: S,
    // ranking_policy: C: C,
    player_repository: PlayerRepository<M>,
    threads: usize,
}

impl<M, P, S, /*C*/> Tournament<M, P, S, /*C*/>
    where
        M: Match + 'static,
        P: Scheduler,
        S: ScoringSystem<M>,
        // C: RankingPolicy,
{
    pub fn new(
        scheduler: P,
        scoring_policy: S,
        // ranking_policy: C: C,
        player_repository: PlayerRepository<M>,
    ) -> Self {
        Tournament {
            scheduler,
            scoring_policy,
            // ranking_policy: C,
            player_repository,
            threads: 1,
        }
    }

    pub fn with_threads(self, count: usize) -> Self {
        Tournament {
            threads: count,
            ..self
        }
    }

    pub fn run(mut self) {
        let (tx, results) = mpsc::channel();
        let (mut pairs, rx2) = spmc::channel();

        let stop_signal = Arc::new(AtomicBool::new(false));
        let repo = Arc::new(self.player_repository);

        let mut executors: Vec<JoinHandle<Result<TerminationCondition, Error>>> = Vec::new();

        for _ in 0..self.threads {
            let executor: Executor<M> =
                Executor::new(rx2.clone(), tx.clone(), repo.clone(), stop_signal.clone());
            executors.push(thread::spawn(move || executor.run()));
        }

        // keep executors busy
        for _ in 0..self.threads {
            let pair = self.scheduler.get();
            pairs.send(pair).expect("Error dispatching pairs");
        }

        loop {
            let match_result = results.recv().expect("Error receiving match results");

            // report the result
            self.scoring_policy.report(match_result);

            // now queue the next pair
            let pair = self.scheduler.get();
            pairs.send(pair).expect("Error dispatching pairs");

            if pair.is_none() {
                break;
            }
        }

        // queue n-1 None values to end the executors
        for _ in 0..self.threads-1 {
            pairs.send(None).expect("Error dispatching finalising pairs");
        }

        // stop the executors
        stop_signal.store(true, Ordering::Relaxed);
        for handle in executors {
            match handle.join().expect("Failed to join thread") {
                Ok(t) => {
                    println!("Termination condition: {:?}", t);
                }
                Err(e) => {
                    eprintln!("Error in executor: {}", e);
                }
            }
        }
    }
}

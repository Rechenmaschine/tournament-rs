use crate::player_repository::PlayerRepository;
use crate::scheduling::Scheduler;
use crate::{Match, ScoringSystem};
use anyhow::Error;
use std::sync::mpsc;
use threadpool::ThreadPool;

pub struct Tournament<M, P, S /*C*/>
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

impl<M, P, S /*C*/> Tournament<M, P, S /*C*/>
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

    pub fn run(&mut self) {
        let threadpool = ThreadPool::new(self.threads);
        let (tx, results) = mpsc::channel::<Result<M::MatchResult, Error>>();

        // initialise the scheduler
        self.scheduler.init();

        loop {
            // Check if scheduler is blocking
            if let Ok(pair) = self.scheduler.try_get() {
                match pair {
                    None => {
                        break;
                    }
                    Some((p1, p2)) => {
                        let tx = tx.clone();
                        let p1 = self.player_repository.get(p1).expect("Player not found");
                        let p2 = self.player_repository.get(p2).expect("Player not found");

                        // Submit the match to be executed
                        threadpool.execute(move || {
                            let mut match_ = M::new(p1, p2);
                            let result = match_.playout();
                            tx.send(result).expect("Error sending match result");
                        });
                    }
                }
            } else {
                // scheduler is blocking, so we wait on the results
                let match_result = results.recv().expect("Error receiving match result");

                if let Ok(result) = match_result {
                    // report the result
                    self.scoring_policy.report(result);
                } else {
                    eprintln!("{}", match_result.unwrap_err());
                }
            }
        }
    }
}

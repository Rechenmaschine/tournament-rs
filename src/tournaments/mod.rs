use crate::game::{LabelledOutcome, Match};
use crate::player::PlayerRepository;
use crate::prelude::MatchResult;
use crate::scheduling::Scheduler;
use crate::scoring::ScoringSystem;
use anyhow::Error;
use std::sync::mpsc;
use threadpool::ThreadPool;

pub struct Tournament<M, P, S>
where
    M: Match,
    P: Scheduler,
    S: ScoringSystem<M>,
{
    scheduler: P,
    scoring_policy: S,
    player_repository: PlayerRepository<M>,
    thread_pool: ThreadPool,
    active_matches: usize,
    channel: (
        mpsc::Sender<Result<(LabelledOutcome, M::MatchResult), Error>>,
        mpsc::Receiver<Result<(LabelledOutcome, M::MatchResult), Error>>,
    ),
}

impl<M, P, S> Tournament<M, P, S>
where
    M: Match + 'static,
    P: Scheduler,
    S: ScoringSystem<M>,
{
    pub fn new(scheduler: P, scoring_policy: S, player_repository: PlayerRepository<M>) -> Self {
        Tournament {
            scheduler,
            scoring_policy,
            player_repository,
            thread_pool: ThreadPool::new(1),
            active_matches: 0,
            channel: mpsc::channel::<Result<(LabelledOutcome, M::MatchResult), Error>>(),
        }
    }

    pub fn set_threads(&mut self, count: usize) {
        self.thread_pool.set_num_threads(count);
    }

    pub fn init(&mut self) {
        // initialise the scheduler
        self.scheduler.init();
    }
}

impl<M, P, S> Iterator for Tournament<M, P, S>
where
    M: Match + 'static,
    P: Scheduler,
    S: ScoringSystem<M>,
{
    type Item = Result<(LabelledOutcome, M::MatchResult), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let (tx, results) = &self.channel;

        while let Ok(pair) = self.scheduler.try_get() {
            // Poll finished matches before queueing more. In a non finite tournament, this will
            // ensure that we don't poll the scheduler forever.
            if self.active_matches >= self.thread_pool.max_count() {
                break;
            }

            match pair {
                None => {
                    break;
                }
                Some((p1, p2)) => {
                    let tx = tx.clone();

                    let p1 = self
                        .player_repository
                        .get_instance(p1)
                        .expect("Player not found");
                    let p2 = self
                        .player_repository
                        .get_instance(p2)
                        .expect("Player not found");

                    let p1_d = p1.data();
                    let p2_d = p2.data();

                    // Submit the match to be executed
                    self.thread_pool.execute(move || {
                        let mut match_ = M::new(p1, p2);
                        let result = match_.playout();

                        let result_data = result.map(|result| {
                            (LabelledOutcome::new(result.outcome(), p1_d, p2_d), result)
                        });

                        tx.send(result_data).expect("Error sending match result")
                    });

                    self.active_matches += 1;
                }
            }
        }

        if self.active_matches == 0 {
            return None;
        }

        // scheduler is blocking, so we wait on the results
        let match_result = results.recv().expect("Error receiving match result");
        self.active_matches -= 1;

        if let Ok((ref outcome, ref result)) = match_result {
            // report the result
            self.scoring_policy.report(&outcome, &result);
        } else {
            // TODO Handle the error
        }

        Some(match_result)
    }
}

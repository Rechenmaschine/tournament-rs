#![allow(dead_code)]
#![allow(unused)]

use glasswing::agents::minimax_agent::MiniMaxAgent;
use glasswing::agents::random_agent::RandomAgent;
use glasswing::agents::simple_agent::SimpleAgent;
use glasswing::core::{Agent, Contest, IntoAgent, Player as GlasswingPlayer};
use glasswing::games::counting_game::{CountingGame, CountingGameEvaluator};
use rand::rngs::{StdRng, ThreadRng};
use rand::{thread_rng, SeedableRng};
use std::time::Duration;
use tournament_rs::game::LabelledOutcome;
use tournament_rs::player::{PlayerGen, PlayerRepository};
use tournament_rs::prelude::{MatchResult, Tournament};
use tournament_rs::scheduling::RoundRobbinScheduler;
use tournament_rs::scoring::DefaultScoring;

#[test]
fn hello_world() {
    let mut players = PlayerRepository::<Contest<CountingGame>>::new();

    players.add_player(
        PlayerGen::new(|| GlasswingPlayer::new(SimpleAgent::new().boxed()))
            .with_name("Simple-1"),
    );

    players.add_player(
        PlayerGen::new(|| {
            GlasswingPlayer::new(MiniMaxAgent::new(15, CountingGameEvaluator).boxed())
                .with_name("Minimax-1")
                .with_time_limit(Duration::from_millis(100))
        })
        .with_name("Minimax-1"),
    );

    let mut tournament = Tournament::new(
        RoundRobbinScheduler::new(players.ids()),
        DefaultScoring::new(players.ids()),
        players,
    );

    tournament.init();
    for x in tournament {
        match x {
            Ok((outcome, _)) => match outcome {
                LabelledOutcome::Win { winner, loser } => {
                    println!("{:?} beat {:?}", winner, loser);
                }
                LabelledOutcome::Draw(p1, p2) => {
                    println!("{:?} drew with {:?}", p1, p2);
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

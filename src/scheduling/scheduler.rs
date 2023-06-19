use crate::PlayerId;

/// A scheduler is responsible for pairing players for a match.
pub trait Scheduler {
    /// Called once at the beginning of the tournament.
    /// Used to perform any necessary initialization.
    fn init(&mut self) {}

    /// Hint to the pairing policy that a new round is starting.
    fn start_round(&mut self) {}

    /// Returns the next pairing of player to play a match between, or None to notify executors
    /// that the tournament is over, the round has ended, or no more pairings are available.
    ///
    /// This method may block, until a pairing is available. This can happen
    /// due to dependent matches, ie. the Scheduler must wait for a previous match
    /// to complete in order to determine the next pairing.
    ///
    fn get(&mut self) -> Option<(PlayerId, PlayerId)>;
}

/// This trait marks a scheduler, that balances the number of times each agent plays as
/// Player 1 and Player 2. It is ideal for games where one player is at a disadvantage.
///
/// Formally, let `n1` and `n2` be the number of times a player plays as Player 1 and Player 2,
/// respectively. Then this scheduler guarantees that:
/// |n1 - n2| ≤ 1
///
/// Note however, that this scheduler does not necessarily guarantee the *best* assignment.
trait PlayerBalancing {}

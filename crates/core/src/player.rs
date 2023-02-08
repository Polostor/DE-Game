use core::fmt;
use std::cmp::Ordering;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Component, PartialEq, Eq, Hash)]
pub enum Player {
    Player1,
    Player2,
    Player3,
    Player4,
}

impl Player {
    pub fn to_num(self) -> u8 {
        match self {
            Self::Player1 => 1,
            Self::Player2 => 2,
            Self::Player3 => 3,
            Self::Player4 => 4,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "player {}", self.to_num())
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_num().partial_cmp(&other.to_num())
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct PlayerRange {
    start: Player,
    stop: Player,
}

impl PlayerRange {
    /// Returns inclusive player range from first player to `stop`.
    pub fn up_to(stop: Player) -> Self {
        Self::new(Player::Player1, stop)
    }

    /// # Arguments
    ///
    /// * `start` - first player, inclusive.
    ///
    /// * `stop` - last player, inclusive.
    pub fn new(start: Player, stop: Player) -> Self {
        assert!(start <= stop);
        Self { start, stop }
    }

    pub fn contains(&self, player: Player) -> bool {
        self.start <= player && player <= self.stop
    }
}

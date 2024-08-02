// src/game/player.rs
use std::net::SocketAddr;

#[derive(Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub score: u32,
    pub address: SocketAddr,
}

impl Player {
    pub fn new(id: String, name: String, address: SocketAddr) -> Self {
        Self {
            id,
            name,
            score: 0,
            address,
        }
    }

    pub fn add_score(&mut self, points: u32) {
        self.score += points;
    }
}
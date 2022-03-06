#![warn(clippy::all, clippy::pedantic)]
use crate::prelude::*;

/// The camera acts as your game's window into the world
pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    const HALF_WIDTH: i32 = DISPLAY_WIDTH / 2;
    const HALF_HEIGHT: i32 = DISPLAY_HEIGHT / 2;

    pub fn new(player_position: Point) -> Self {
        Self {
            left_x: player_position.x - Self::HALF_WIDTH,
            right_x: player_position.x + Self::HALF_WIDTH,
            top_y: player_position.y - Self::HALF_HEIGHT,
            bottom_y: player_position.y + Self::HALF_HEIGHT,
        }
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        self.left_x = player_position.x - Self::HALF_WIDTH;
        self.right_x = player_position.x + Self::HALF_WIDTH;
        self.top_y = player_position.y - Self::HALF_HEIGHT;
        self.bottom_y = player_position.y + Self::HALF_HEIGHT;
    }
}

#![warn(clippy::all, clippy::pedantic)]
use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            // Randomly spawn one of four different monster types
            glyph: match rng.range(0, 4) {
                0 => to_cp437('E'), // Ettin
                1 => to_cp437('O'), // Ogre
                2 => to_cp437('o'), // Orc
                _ => to_cp437('g'), // Goblin
            },
        },
    ));
}

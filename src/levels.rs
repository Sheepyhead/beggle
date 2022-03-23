use std::fmt;

use bevy::prelude::*;

use crate::GameState;

pub mod level1;

pub struct Levels;

impl Plugin for Levels {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(spawn_level));
    }
}

fn spawn_level(mut commands: Commands, level: Res<CurrentLevel>) {
    (level.spawn)(&mut commands);
}

#[derive(Component)]
struct Peg;

pub(crate) struct CurrentLevel {
    spawn: fn(&mut Commands),
}

pub(crate) struct CurrentBalls(u32);

impl Default for CurrentBalls {
    fn default() -> Self {
        Self(10)
    }
}

impl CurrentBalls {
    pub(crate) fn decrement(&mut self) {
        if self.has_any() {
            self.0 -= 1;
        }
    }

    pub(crate) fn has_any(&self) -> bool {
        self.0 > 0
    }
}

impl fmt::Display for CurrentBalls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
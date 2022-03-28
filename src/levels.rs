use std::fmt;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    ball_shooter::{Ball, BallShooter},
    gui::Hud,
    GameState,
};

pub mod level1;

pub struct Levels;

impl Plugin for Levels {
    fn build(&self, app: &mut App) {
        app.add_state(LevelState::Aiming)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(spawn_level))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(Peg::hit)
                    .with_system(Peg::recolor_on_hit),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(despawn_level))
            .add_system_set(SystemSet::on_exit(LevelState::Dropping).with_system(Peg::despawn));
    }
}

fn spawn_level(mut commands: Commands, level: Res<CurrentLevel>) {
    commands.insert_resource(Score::default());
    (level.spawn)(&mut commands);
}

fn despawn_level(
    mut commands: Commands,
    entities_to_despawn: Query<Entity, Or<(With<Peg>, With<Ball>, With<Hud>, With<BallShooter>)>>,
) {
    for entity in entities_to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Clone, Copy)]
pub enum PegStatus {
    NotHit,
    Hit,
}

#[derive(Clone, Copy)]
pub enum PegType {
    Basic,
}

#[derive(Component)]
pub struct Peg {
    pub status: PegStatus,
    pub typ: PegType,
}

impl Default for Peg {
    fn default() -> Self {
        Self {
            status: PegStatus::NotHit,
            typ: PegType::Basic,
        }
    }
}

impl Peg {
    fn hit(mut events: EventReader<ContactEvent>, mut pegs: Query<&mut Peg>) {
        for event in events.iter() {
            if let ContactEvent::Started(h1, h2) = event {
                let mut peg = pegs.get_mut(h1.entity());
                if peg.is_err() {
                    peg = pegs.get_mut(h2.entity());
                }
                if let Ok(mut peg) = peg {
                    if let PegStatus::NotHit = peg.status {
                        peg.status = PegStatus::Hit;
                    }
                }
            }
        }
    }

    fn recolor_on_hit(mut pegs: Query<(&Peg, &mut ColliderDebugRender), Changed<Peg>>) {
        for (peg, mut render) in pegs.iter_mut() {
            render.color = match peg.status {
                PegStatus::NotHit => Color::YELLOW,
                PegStatus::Hit => Color::GREEN,
            }
        }
    }

    fn despawn(mut commands: Commands, mut score: ResMut<Score>, pegs: Query<(Entity, &Peg)>) {
        for (entity, peg) in pegs.iter() {
            if let PegStatus::Hit = peg.status {
                score.points += 1;
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum LevelState {
    Aiming,
    Dropping,
}

#[derive(Default)]
pub struct Score {
    pub points: u64,
}

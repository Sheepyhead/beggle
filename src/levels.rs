use std::fmt;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    ball_shooter::{Ball, BallShooter},
    gui::Hud,
    GameState,
};

use self::level1::Level1;

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
    let pegs = (level.spawn)(&mut commands);
    let range = 0..pegs.len();
    let (goal_peg_1, mut goal_peg_2) = (
        fastrand::usize(range.clone()),
        fastrand::usize(range.clone()),
    );
    while goal_peg_1 == goal_peg_2 {
        goal_peg_2 = fastrand::usize(range.clone());
    }
    commands.entity(pegs[goal_peg_1]).insert(Peg {
        typ: PegType::Goal,
        ..Peg::default()
    });
    commands.entity(pegs[goal_peg_2]).insert(Peg {
        typ: PegType::Goal,
        ..Peg::default()
    });
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
    Goal,
}

impl From<PegType> for Color {
    fn from(val: PegType) -> Self {
        match val {
            PegType::Basic => Color::BLUE,
            PegType::Goal => Color::ORANGE_RED,
        }
    }
}

impl PegType {
    fn points(&self) -> u32 {
        match self {
            PegType::Basic => 10,
            PegType::Goal => 100,
        }
    }
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
    fn hit(
        mut events: EventReader<ContactEvent>,
        mut score: ResMut<Score>,
        mut pegs: Query<&mut Peg>,
    ) {
        for event in events.iter() {
            if let ContactEvent::Started(h1, h2) = event {
                let mut peg = pegs.get_mut(h1.entity());
                if peg.is_err() {
                    peg = pegs.get_mut(h2.entity());
                }
                if let Ok(mut peg) = peg {
                    if let PegStatus::NotHit = peg.status {
                        score.points += peg.points() as u64;
                        peg.status = PegStatus::Hit;
                    }
                }
            }
        }
    }

    fn recolor_on_hit(mut pegs: Query<(&Peg, &mut ColliderDebugRender), Changed<Peg>>) {
        for (peg, mut render) in pegs.iter_mut() {
            render.color = peg.typ.into();
            if let PegStatus::Hit = peg.status {
                render.color = Color::rgb(
                    render.color.r() + 0.7,
                    render.color.g() + 0.7,
                    render.color.b() + 0.7,
                );
            }
        }
    }

    fn despawn(mut commands: Commands, pegs: Query<(Entity, &Peg)>) {
        for (entity, peg) in pegs.iter() {
            if let PegStatus::Hit = peg.status {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    pub fn points(&self) -> u32 {
        self.typ.points()
    }
}

#[derive(Clone, Copy)]
pub(crate) struct CurrentLevel {
    spawn: fn(&mut Commands) -> Vec<Entity>,
}

impl TryInto<CurrentLevel> for u32 {
    type Error = ();

    fn try_into(self) -> Result<CurrentLevel, Self::Error> {
        match self {
            1 => Ok(Level1.into()),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
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

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.points.fmt(f)
    }
}

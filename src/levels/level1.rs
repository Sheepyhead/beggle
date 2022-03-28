use bevy::{math::Vec2, prelude::*};
use bevy_rapier2d::prelude::*;

use super::{CurrentBalls, CurrentLevel, Peg};

pub(crate) struct Level1;

impl From<Level1> for CurrentLevel {
    fn from(_: Level1) -> Self {
        CurrentLevel {
            spawn: |commands: &mut Commands| {
                let columns = 12;
                let rows = 8;
                for column in 0..columns {
                    for row in 0..rows {
                        let horizontal_offset = 40.0;
                        let vertical_offset = 40.0;
                        let radius = 10.0;
                        let x = column as f32;
                        let y = row as f32;
                        let columns = columns as f32;
                        let rows = rows as f32;
                        commands
                            .spawn_bundle(ColliderBundle {
                                shape: ColliderShape::ball(radius).into(),
                                ..ColliderBundle::default()
                            })
                            .insert_bundle(RigidBodyBundle {
                                body_type: RigidBodyType::Static.into(),
                                position: Vec2::new(
                                    x + (horizontal_offset * x) + radius
                                        - (horizontal_offset * columns) / 2.0
                                        - if row % 2 == 0 {
                                            horizontal_offset / 2.0
                                        } else {
                                            0.0
                                        },
                                    y + (vertical_offset * y) - (vertical_offset * rows) / 2.0,
                                )
                                .into(),
                                ..RigidBodyBundle::default()
                            })
                            .insert_bundle((
                                Peg::default(),
                                RigidBodyPositionSync::Discrete,
                                ColliderDebugRender {
                                    color: Color::YELLOW,
                                },
                            ));
                    }
                }
                commands.insert_resource(CurrentBalls::default());
            },
        }
    }
}

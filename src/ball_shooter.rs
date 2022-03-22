use crate::{GameState, MainCamera};
use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    math::Vec3Swizzles,
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub(crate) struct BallShooter;

impl Plugin for BallShooter {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(Self::spawn))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(Self::turn_toward_cursor)
                    .with_system(Self::shoot_ball),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(Self::despawn));
    }
}

#[derive(Component)]
struct BallShooterBase;

impl BallShooter {
    fn spawn(mut commands: Commands) {
        commands
            .spawn_bundle((
                Transform::from_xyz(0.0, 330.0, 0.0),
                GlobalTransform::default(),
                BallShooterBase,
            ))
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.75, 0.75, 0.75),
                            custom_size: Some(Vec2::new(20.0, 50.0)),
                            ..Sprite::default()
                        },
                        transform: Transform::from_xyz(0.0, -100.0, 0.0),
                        ..SpriteBundle::default()
                    })
                    .insert(BallShooter);
            });
    }

    fn despawn(mut commands: Commands, shooter: Query<Entity, With<BallShooter>>) {
        for entity in shooter.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    fn turn_toward_cursor(
        windows: Res<Windows>,
        mut shooters: Query<(&mut Transform, &GlobalTransform), With<BallShooterBase>>,
        camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ) {
        if let Some(window) = windows.get_primary() {
            for (mut local, global) in shooters.iter_mut() {
                if let Some(cursor_position) = window.cursor_position() {
                    let (camera, camera_position) = camera.single();
                    // get the size of the window
                    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

                    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
                    let ndc = (cursor_position / window_size) * 2.0 - Vec2::ONE;

                    // matrix for undoing the projection and camera transform
                    let ndc_to_world =
                        camera_position.compute_matrix() * camera.projection_matrix.inverse();

                    // use it to convert ndc to world-space coordinates
                    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0)).xy();

                    // Calculate the angle of rotation between the transform and the mouse cursor position
                    let angle = f32::atan2(
                        world_pos.y - global.translation.y,
                        world_pos.x - global.translation.x,
                    ) + PI / 2.0;

                    local.rotation = Quat::from_rotation_z(angle);
                }
            }
        }
    }

    fn shoot_ball(
        mut commands: Commands,
        mut events: EventReader<MouseButtonInput>,
        shooters: Query<&mut GlobalTransform, With<BallShooter>>,
    ) {
        for event in events.iter() {
            if let MouseButtonInput {
                button: MouseButton::Left,
                state: ElementState::Pressed,
            } = event
            {
                for shooter in shooters.iter() {
                    let angle = shooter.rotation.to_euler(EulerRot::XYZ).2 - PI / 2.0;
                    commands
                        .spawn_bundle(RigidBodyBundle {
                            position: shooter.translation.xyz().into(),
                            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
                            forces: RigidBodyForces {
                                gravity_scale: 10.0,
                                ..RigidBodyForces::default()
                            }
                            .into(),
                            velocity: RigidBodyVelocity {
                                linvel: (Vec2::new(angle.cos(), angle.sin()) * 1000.0).into(),
                                ..RigidBodyVelocity::default()
                            }
                            .into(),
                            ..RigidBodyBundle::default()
                        })
                        .insert_bundle(ColliderBundle {
                            shape: ColliderShape::ball(7.5).into(),
                            ..ColliderBundle::default()
                        })
                        .insert_bundle((
                            Ball,
                            RigidBodyPositionSync::Discrete,
                            ColliderDebugRender { color: Color::RED },
                        ));
                }
            }
        }
    }
}

#[derive(Component)]
struct Ball;

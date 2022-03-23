use bevy::prelude::*;

use crate::{ball_shooter::Ball, levels::CurrentBalls, GameState};

#[derive(Component)]
pub(crate) struct GameOver;

impl Plugin for GameOver {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(GameOver::trigger))
            .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(GameOver::spawn))
            .add_system_set(
                SystemSet::on_update(GameState::GameOver).with_system(GameOver::trigger_despawn),
            )
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(GameOver::despawn));
    }
}

impl GameOver {
    fn trigger(
        mut game_state: ResMut<State<GameState>>,
        current_balls: Res<CurrentBalls>,
        removed: RemovedComponents<Ball>,
        balls: Query<(), With<Ball>>,
    ) {
        if removed.iter().count() >= balls.iter().count() && !current_balls.has_any() {
            game_state.set(GameState::GameOver).unwrap();
        }
    }

    fn spawn(mut commands: Commands, ass: Res<AssetServer>) {
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    margin: Rect::all(Val::Auto),
                    ..Style::default()
                },
                visibility: Visibility { is_visible: false },
                ..NodeBundle::default()
            })
            .insert(GameOver)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Game Over!!",
                        TextStyle {
                            font: ass.load("fonts/Roboto-Regular.ttf"),
                            font_size: 50.0,
                            color: Color::ANTIQUE_WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..TextBundle::default()
                });
            });

        commands.insert_resource(DespawnTimer(Timer::from_seconds(5.0, false)));
    }

    fn trigger_despawn(
        mut game_state: ResMut<State<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<DespawnTimer>,
    ) {
        if timer.0.tick(time.delta()).just_finished() {
            game_state.set(GameState::MainMenu).unwrap();
        }
    }

    fn despawn(mut commands: Commands, game_over: Query<Entity, With<GameOver>>) {
        for entity in game_over.iter() {
            commands.entity(entity).despawn_recursive();
        }
        commands.remove_resource::<DespawnTimer>();
    }
}

struct DespawnTimer(Timer);

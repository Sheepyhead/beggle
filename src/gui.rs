use bevy::prelude::*;

use crate::{
    levels::{CurrentBalls, Score},
    GameState,
};

pub(crate) struct Gui;

impl Plugin for Gui {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(Gui::spawn))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(Gui::update))
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(Gui::despawn));
    }
}

#[derive(Component)]
pub(crate) enum Hud {
    BallsText,
    ScoreText,
}

impl Gui {
    fn spawn(
        mut commands: Commands,
        ass: Res<AssetServer>,
    ) {
        commands
            .spawn_bundle(TextBundle {
                text: Text::with_section(
                    "",
                    TextStyle {
                        font: ass.load("fonts/Roboto-Regular.ttf"),
                        font_size: 20.0,
                        color: Color::ANTIQUE_WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..TextBundle::default()
            })
            .insert(Hud::BallsText);
        commands
            .spawn_bundle(TextBundle {
                text: Text::with_section(
                    "",
                    TextStyle {
                        font: ass.load("fonts/Roboto-Regular.ttf"),
                        font_size: 20.0,
                        color: Color::ANTIQUE_WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..TextBundle::default()
            })
            .insert(Hud::ScoreText);
    }

    fn update(
        mut gui: Query<(&mut Text, &Hud)>,
        current_balls: Res<CurrentBalls>,
        score: Res<Score>,
    ) {
        for (mut text, hud) in gui.iter_mut() {
            match hud {
                Hud::BallsText => {
                    if current_balls.is_changed() {
                        text.sections[0].value = format!("Balls left: {}", *current_balls);
                    }
                }
                Hud::ScoreText => {
                    if score.is_changed() {
                        text.sections[0].value = format!("Score: {}", *score);
                    }
                }
            }
        }
    }

    fn despawn(mut commands: Commands, gui: Query<Entity, With<Hud>>) {
        for entity in gui.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

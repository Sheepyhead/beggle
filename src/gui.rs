use bevy::prelude::*;

use crate::levels::CurrentBalls;

pub(crate) struct Gui;

impl Plugin for Gui {
    fn build(&self, app: &mut App) {
        app.add_system(Gui::spawn);
    }
}

#[derive(Component)]
pub(crate) enum Hud {
    BallsText,
}

impl Gui {
    fn spawn(
        mut commands: Commands,
        current_balls: Option<Res<CurrentBalls>>,
        ass: Res<AssetServer>,
        mut gui: Query<(&mut Text, &Hud)>,
    ) {
        if let Some(current_balls) = current_balls {
            if current_balls.is_changed() {
                if gui.is_empty() {
                    commands
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                format!("Balls left: {}", *current_balls),
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
                } else {
                    for (mut text, hud) in gui.iter_mut() {
                        match hud {
                            Hud::BallsText => {
                                text.sections[0].value = format!("Balls left: {}", *current_balls)
                            }
                        }
                    }
                }
            }
        }
    }
}

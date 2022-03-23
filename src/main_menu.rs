use bevy::{app::AppExit, prelude::*};

use crate::{GameState, levels::{CurrentLevel, level1::Level1}};

#[derive(Component)]
pub(crate) struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(Self::spawn))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu).with_system(Self::handle_buttons),
            )
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(Self::despawn));
    }
}

#[derive(Component)]
enum MainMenuButton {
    Play,
    Exit,
}

impl MainMenu {
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
            .insert(MainMenu)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Welcome to Beggle!",
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
                parent
                    .spawn_bundle(ButtonBundle {
                        focus_policy: bevy::ui::FocusPolicy::Block,
                        color: Color::ANTIQUE_WHITE.into(),
                        style: Style {
                            margin: Rect {
                                top: Val::Px(50.0),
                                bottom: Val::Px(50.0),
                                ..Rect::default()
                            },
                            ..Style::default()
                        },
                        ..ButtonBundle::default()
                    })
                    .insert(MainMenuButton::Play)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Play",
                                TextStyle {
                                    font: ass.load("fonts/Roboto-Regular.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                                TextAlignment {
                                    vertical: VerticalAlign::Center,
                                    horizontal: HorizontalAlign::Center,
                                },
                            ),
                            style: Style {
                                margin: Rect::all(Val::Px(10.0)),
                                ..Style::default()
                            },
                            ..TextBundle::default()
                        });
                    });
                parent
                    .spawn_bundle(ButtonBundle {
                        focus_policy: bevy::ui::FocusPolicy::Block,
                        color: Color::ANTIQUE_WHITE.into(),
                        ..ButtonBundle::default()
                    })
                    .insert(MainMenuButton::Exit)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Exit",
                                TextStyle {
                                    font: ass.load("fonts/Roboto-Regular.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                                TextAlignment {
                                    vertical: VerticalAlign::Center,
                                    horizontal: HorizontalAlign::Center,
                                },
                            ),
                            style: Style {
                                margin: Rect::all(Val::Px(10.0)),
                                ..Style::default()
                            },
                            ..TextBundle::default()
                        });
                    });
            });
    }

    fn despawn(mut commands: Commands, menu: Query<Entity, With<MainMenu>>) {
        for entity in menu.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    fn handle_buttons(
        mut commands: Commands,
        mut game_state: ResMut<State<GameState>>,
        mut events: EventWriter<AppExit>,
        butts: Query<(&MainMenuButton, &Interaction), Changed<Interaction>>,
    ) {
        for butt in butts.iter() {
            match butt {
                (MainMenuButton::Play, Interaction::Clicked) => {
                    commands.insert_resource::<CurrentLevel>(Level1.into());
                    game_state.set(GameState::Game).unwrap();
                }
                (MainMenuButton::Play, Interaction::Hovered) => {}
                (MainMenuButton::Exit, Interaction::Clicked) => events.send(AppExit),
                (MainMenuButton::Exit, Interaction::Hovered) => {}
                _ => {}
            }
        }
    }
}

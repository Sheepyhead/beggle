use bevy::prelude::*;

use crate::{levels::CurrentLevel, GameState};

#[derive(Component)]
pub(crate) struct LevelSelect;

impl Plugin for LevelSelect {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::LevelSelect).with_system(LevelSelect::spawn),
        )
        .add_system_set(
            SystemSet::on_update(GameState::LevelSelect)
                .with_system(LevelSelect::button_color)
                .with_system(LevelSelect::button_press),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::LevelSelect).with_system(LevelSelect::despawn),
        );
    }
}

#[derive(Component)]
struct LevelButton(Option<CurrentLevel>);

impl LevelSelect {
    fn spawn(mut commands: Commands, ass: Res<AssetServer>) {
        let mut children = vec![];
        let rows = 5;
        let columns = 8;
        let mut level = 1;
        for _ in 0..rows {
            let mut buttons = vec![];

            for _ in 0..columns {
                buttons.push(
                    commands
                        .spawn_bundle(ButtonBundle {
                            focus_policy: bevy::ui::FocusPolicy::Block,
                            color: Color::DARK_GRAY.into(),
                            style: {
                                Style {
                                    size: Size {
                                        width: Val::Px(75.0),
                                        height: Val::Px(75.0),
                                    },
                                    align_items: AlignItems::Center,
                                    align_content: AlignContent::Center,
                                    justify_content: JustifyContent::Center,
                                    margin: Rect::all(Val::Px(10.0)),
                                    ..Style::default()
                                }
                            },
                            ..ButtonBundle::default()
                        })
                        .insert(LevelButton(level.try_into().ok()))
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    format!("Level {level}"),
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
                                ..TextBundle::default()
                            });
                        })
                        .id(),
                );

                level += 1;
            }

            children.push(
                commands
                    .spawn_bundle(NodeBundle {
                        visibility: Visibility { is_visible: false },
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            align_content: AlignContent::Center,
                            margin: Rect::all(Val::Auto),
                            ..Style::default()
                        },
                        ..NodeBundle::default()
                    })
                    .push_children(&buttons)
                    .id(),
            );
        }
        commands
            .spawn_bundle(NodeBundle {
                visibility: Visibility { is_visible: false },
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    margin: Rect::all(Val::Auto),
                    ..Style::default()
                },
                ..NodeBundle::default()
            })
            .insert(LevelSelect)
            .push_children(&children);
    }

    fn despawn(mut commands: Commands, level_select: Query<Entity, With<LevelSelect>>) {
        for entity in level_select.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    fn button_color(mut butts: Query<(&mut UiColor, &LevelButton), Changed<LevelButton>>) {
        for (mut color, LevelButton(butt)) in butts.iter_mut() {
            *color = if butt.is_some() {
                Color::ANTIQUE_WHITE
            } else {
                Color::DARK_GRAY
            }
            .into()
        }
    }

    fn button_press(
        mut commands: Commands,
        mut game_state: ResMut<State<GameState>>,
        butts: Query<(&Interaction, &LevelButton), Changed<Interaction>>,
    ) {
        for (interaction, LevelButton(level)) in butts.iter() {
            if let (Interaction::Clicked, Some(level)) = (interaction, level) {
                game_state.set(GameState::Game).unwrap();
                commands.insert_resource(*level);
            }
        }
    }
}

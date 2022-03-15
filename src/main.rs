use bevy::{app::AppExit, prelude::*};
use bevy_editor_pls::EditorPlugin;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_state(GameState::MainMenu)
        .add_startup_system(setup_camera)
        .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(render_main_menu))
        .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(main_menu_buttons))
        .run();
}

fn setup_camera(mut commands: Commands, mut color: ResMut<ClearColor>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    color.0 = Color::BLACK;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum GameState {
    MainMenu,
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
enum MainMenuButton {
    Play,
    Exit,
}

fn render_main_menu(mut commands: Commands, ass: Res<AssetServer>) {
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

fn main_menu_buttons(
    mut events: EventWriter<AppExit>,
    butts: Query<(&MainMenuButton, &Interaction), Changed<Interaction>>,
) {
    for butt in butts.iter() {
        match butt {
            (MainMenuButton::Play, Interaction::Clicked) => println!("Play!"),
            (MainMenuButton::Play, Interaction::Hovered) => {}
            (MainMenuButton::Exit, Interaction::Clicked) => events.send(AppExit),
            (MainMenuButton::Exit, Interaction::Hovered) => {}
            _ => {}
        }
    }
}

#![allow(clippy::type_complexity)]

use ball_shooter::BallShooter;
use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_rapier2d::prelude::*;
use game_over::GameOver;
use gui::Gui;
use level_select::LevelSelect;
use levels::Levels;
use main_menu::MainMenu;

mod ball_shooter;
mod game_over;
mod gui;
mod level_select;
mod levels;
mod main_menu;
mod workarounds;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(MainMenu)
        .add_plugin(BallShooter)
        .add_plugin(Levels)
        .add_plugin(LevelSelect)
        .add_plugin(Gui)
        .add_plugin(GameOver)
        .add_state(GameState::MainMenu)
        .add_startup_system(setup_camera)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands, mut color: ResMut<ClearColor>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands.spawn_bundle(UiCameraBundle::default());
    color.0 = Color::BLACK;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum GameState {
    MainMenu,
    LevelSelect,
    Game,
    GameOver,
}

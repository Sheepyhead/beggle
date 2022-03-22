use ball_shooter::BallShooter;
use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_rapier2d::prelude::*;
use main_menu::MainMenu;

mod ball_shooter;
mod main_menu;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(MainMenu)
        .add_plugin(BallShooter)
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
    Game,
}

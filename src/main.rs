use bevy::prelude::*;
pub mod character;
use character::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(character::CharacterPlugin)
        .add_systems(Startup, spawn)
        .run();
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite::from_image(asset_server.load("core/res/test.png")),
        components::Velocity::default(),
        components::MoveSpeed::default(),
        components::DesiredVelocity::default(),
        components::CharacterController
    ));
}
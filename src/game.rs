use bevy::prelude::*;
pub mod character;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(character::CharacterPlugin)
            .add_systems(Startup, spawn);

    }
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite::from_image(asset_server.load("core/res/test.png")),
        character::components::Velocity::default(),
        character::components::MoveSpeed::default(),
        character::components::DesiredVelocity::default(),
        character::components::CharacterController
    ));
}
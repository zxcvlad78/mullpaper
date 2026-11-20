use bevy::prelude::*;
pub mod client;
pub mod server;
pub mod shared;
use client::*;
use server::*;
use shared::*; 

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn);
    }
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2d);
    commands.spawn(
        character::bundles::CharacterBundle::default()
    );
}
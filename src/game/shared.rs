use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

pub mod character;

pub struct SharedPlugin;
impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(character::CharacterPlugin)

            .register_type::<Transform>()
            .replicate::<Transform>()
    }
}
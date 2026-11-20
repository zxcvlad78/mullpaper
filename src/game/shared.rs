use bevy::prelude::*;
use lightyear::prelude::*;

pub mod character; use character::*; use character::components::*;
pub mod network; use network::*; use network::components::*;

pub struct SharedPlugin;
impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CharacterPlugin)
            .add_plugins(NetworkPlugin)

            .register_type::<NetworkPosition>()
            .component::<NetworkPosition>().replicate();
    }
}
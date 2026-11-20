use bevy::prelude::*;
use lightyear::prelude::*;

pub mod network;

pub struct ServerPlugin;
impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        // app
        //     .add_plugins(CharacterPlugin)
        //     .add_plugins(NetworkPlugin)

        //     .register_type::<NetworkPosition>()
        //     .component::<NetworkPosition>().replicate();
    }
}
use bevy::prelude::*;
use lightyear::prelude::*;


pub struct ClientPlugin;
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        // app
        //     .add_plugins(CharacterPlugin)
        //     .add_plugins(NetworkPlugin)

        //     .register_type::<NetworkPosition>()
        //     .component::<NetworkPosition>().replicate();
    }
}
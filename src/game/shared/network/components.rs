use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, PartialEq, Reflect)]
pub struct NetworkPosition {
    pub vec: Vec3
}

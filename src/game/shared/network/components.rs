use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Component, Serialize, Deserialize, Clone, PartialEq, Reflect)]
pub struct NetworkPosition {
    pub vec: Vec3
}

#[derive(Default, Component, Serialize, Deserialize, Clone, PartialEq, Reflect)]
pub struct InputDirection {
    pub up:    bool,
    pub down:  bool,
    pub left:  bool,
    pub right: bool
}

impl InputDirection {
    pub fn is_none(&self) -> bool {
        !self.up && !self.down && !self.left && !self.right
    }
}
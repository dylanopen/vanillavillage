pub mod testing;
pub mod camera;

use bevy::prelude::*;

use crate::{camera::VanillaCameraPlugin, testing::VanillaTestingPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VanillaCameraPlugin)
        .add_plugins(VanillaTestingPlugin)
        .run();
}




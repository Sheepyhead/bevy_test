use bevy::prelude::*;
use bevy_mod_picking::PickingCamera;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

fn test(q: Query<&PickingCamera>) {}

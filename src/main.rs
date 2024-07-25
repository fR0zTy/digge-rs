mod camera;
use bevy::prelude::*;
use camera::CameraPlugin;
fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .run();
}

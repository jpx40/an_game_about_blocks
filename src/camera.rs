use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
#[derive(Component)]
pub struct MainCamera;

fn setup(mut command: Commands) {
    command.spawn((Camera2d, Msaa::Off, MainCamera));
}

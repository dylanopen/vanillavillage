use bevy::prelude::*;

pub struct VanillaCameraPlugin;

impl Plugin for VanillaCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, move_camera);
    }
}

#[derive(Component)]
pub struct PrimaryCameraMarker;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
            Camera2d,
            PrimaryCameraMarker,
            Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_query: Query<&mut Transform, With<PrimaryCameraMarker>>,
) {
    let mut direction = Vec2::new(0.0, 0.0);

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    
    let normalized_direction = direction.normalize_or_zero();
    let speed = 10.0;
    let velocity = normalized_direction * speed;
    let velocity3d = Vec3::new(velocity.x, velocity.y, 0.0);
    
    for mut camera in camera_query {
        camera.translation += velocity3d;
    }
}


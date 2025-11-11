use bevy::{app::{App, Plugin, Startup}, asset::Assets, camera::Camera2d, color::Color, ecs::system::{Commands, ResMut}, math::{Vec2, primitives::{Annulus, Capsule2d, Circle, CircularSector, CircularSegment, Ellipse, Polyline2d, Rectangle, RegularPolygon, Rhombus, Segment2d, Triangle2d}}, mesh::{Mesh, Mesh2d}, sprite_render::{ColorMaterial, MeshMaterial2d}, transform::components::Transform};
pub struct VanillaTestingPlugin;

impl Plugin for VanillaTestingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_rectangle);
    }
}

const X_EXTENT: f32 = 900.0;

fn spawn_test_rectangle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let shapes = [
        meshes.add(Circle::new(50.0)),
        meshes.add(CircularSector::new(50.0, 1.0)),
        meshes.add(CircularSegment::new(50.0, 1.25)),
        meshes.add(Ellipse::new(25.0, 50.0)),
        meshes.add(Annulus::new(25.0, 50.0)),
        meshes.add(Capsule2d::new(25.0, 50.0)),
        meshes.add(Rhombus::new(75.0, 100.0)),
        meshes.add(Rectangle::new(50.0, 100.0)),
        meshes.add(RegularPolygon::new(50.0, 6)),
        meshes.add(Triangle2d::new(
                Vec2::Y * 50.0,
                Vec2::new(-50.0, -50.0),
                Vec2::new(50.0, -50.0),
        )),
        meshes.add(Segment2d::new(
                Vec2::new(-50.0, 50.0),
                Vec2::new(50.0, -50.0),
        )),
        meshes.add(Polyline2d::new(vec![
                Vec2::new(-50.0, 50.0),
                Vec2::new(0.0, -50.0),
                Vec2::new(50.0, 50.0),
        ])),
        ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn((
                Mesh2d(shape),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(
                    // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    0.0,
                    0.0,
                ),
        ));
    }
}


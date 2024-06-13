use bevy::{color::palettes::css::BLUE, prelude::*};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_floor)
        ;
    }
}

pub fn create_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(
        ColorMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            material: materials.add(Color::from(BLUE)),
            transform: Transform::default().with_scale(Vec3::new(500., 500., 0.01)).with_translation(Vec3::Z * -0.05),
            ..Default::default()
        }
    );
}
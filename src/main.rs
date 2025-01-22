use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {


    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 100.0 })), // Large cube
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("textures/skybox.png")), // Skybox texture
                unlit: true, // No lighting effects
                ..default()
            }),
            transform: Transform::from_scale(Vec3::new(-1.0, -1.0, -1.0)), // Inverted normals
            ..default()
        },
    ));

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(-5.0, 4.0, 5.0),
        ..default()
    });

    // Sphere with texture
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 1.0,
            sectors: 32,
            stacks: 16,
        })),
        transform: Transform {
            translation: Vec3::new(0.0, 1.0, 0.0), // Place at the sphere's surface
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2), // Align to the sphere's top
            ..default()
        },
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("sprites/texture.png")),
            perceptual_roughness: 0.9,
            metallic: 0.0,
            ..default()
        }),
        ..default()
    });
}

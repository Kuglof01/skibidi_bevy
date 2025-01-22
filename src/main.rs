use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, (spawn_planet, spawn_light, spawn_camera)).run();
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0.0,0.0,15.0),
        ..default()
    };
    commands.spawn(camera);
}
fn spawn_light(
    mut commands: Commands,
) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 30000.0,
            ..default()
        },
        transform: Transform::from_xyz(-2.0, 2.5, 5.0),
        ..default()
    };
    commands.spawn(light);
}


fn spawn_planet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,

) {
    let planet_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("sprites/texture.jpg")),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 1.0,
            sectors: 32,
            stacks: 16,
        })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: planet_material,
        ..default()
    });
}
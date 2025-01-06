use std::slice::Windows;
use bevy::app;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
/*
fn main() {
    App::new().add_startup_system(setup)
        .add_system(print_names)
        .run()
}
pub fn setup(mut commands: Commands) {
    commands.spawn( Person {
        name: "Alice".to_string(),

    });
}

pub fn print_names(personquery: Query<&Person>) {
    for person in personquery.iter() {
        println!("Name: {}", person.name);
    }
}
#[derive(Component)]
pub struct Person {
    pub name: String,
}

 */
fn main() {
    App::new().add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .run();
}
#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    ){
let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle{
                transform: Transform::from_xyz(window.width() / 2.0,window.height() / 2.0, 0.0),
                texture:asset_server.load("sprites/ball_blue_large.png"),
                    ..default()


            },
            Player {},
            )
    );
}
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(window.width() / 2.0,window.height() / 2.0, 0.0),
        ..default()
    });
}
//use log::Level;
//use log::{trace, debug};

use std::sync::{Arc, RwLock};
use std::time::Duration;
use bevy::prelude::*;

fn _main() {
    App::new()
        .insert_resource(bevy::log::LogSettings {
             level: bevy::log::Level::TRACE,
             //filter: "wgpu=warn,bevy_ecs=info".to_string(),
             filter: "trace,wgpu=warn,bevy_winit=trace,game_activity=trace,bevy_ecs=info".to_string(),
        })
        /*
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 500.,
            height: 300.,
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        })*/
        .insert_resource(bevy::render::settings::WgpuSettings {
            limits: wgpu::Limits::downlevel_webgl2_defaults(),
            ..default()
        })
        //.insert_resource(Msaa { samples: 2 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}


#[cfg(target_os="android")]
#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    //android_logger::init_once(
    //    android_logger::Config::default().with_min_level(Level::Trace)
    //);

    _main();
}
// Stop rust-analyzer from complaining that this file doesn't have a main() function...
#[cfg(target_os="android")]
#[cfg(allow_unused)]
fn main() {}

#[cfg(not(target_os="android"))]
fn main() {
    //env_logger::builder().filter_level(log::LevelFilter::Warn) // Default Log Level
    //    .parse_default_env()
    //    .init();

    _main();
}
use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{
    diagnostic::LogDiagnosticsPlugin, input::common_conditions::input_just_released, math::DVec3,
    prelude::*, window::WindowResolution,
};
use bevy_avian_baseball_flight::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use blenvy::*;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1024.0;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "The Bullpen".to_string(),
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    }));
    #[cfg(debug_assertions)]
    {
        app.add_plugins(LogDiagnosticsPlugin::default());
        app.add_plugins(PhysicsDebugPlugin::default());
    }

    app.add_plugins(BlenvyPlugin::default());
    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(NoCameraPlayerPlugin);
    app.add_plugins(BaseballFlightPlugin {
        ssw_on: true,
        magnus_on: true,
        drag_on: true,
    });

    app.add_systems(PostStartup, (setup_scene, spawn_camera.after(setup_scene)));

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("fly cam"),
        FlyCam,
        Camera3dBundle {
            camera: Camera {
                is_active: true,
                order: 0,
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ));
}

fn setup_scene(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo::from_path("levels/TheBullpen.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}

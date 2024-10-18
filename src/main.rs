mod pitcher;
mod pitcher_mechanics;
mod prelude;
mod scenes;

use crate::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::ThirdPersonCameraPlugin;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1024.0;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "The Bullpen".to_string(),
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            resizable: false,
            cursor: Cursor {
                grab_mode: CursorGrabMode::Locked,
                ..default()
            },
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.insert_resource(Time::<Fixed>::from_hz(60.0));

    let mut rapier_config = RapierConfiguration::new(1.);
    rapier_config.timestep_mode = TimestepMode::Fixed {
        dt: 1. / 60.,
        substeps: 10,
    };
    // rapier_config.timestep_mode = TimestepMode::Variable {
    //     max_dt: 1.0 / 60.0,
    //     time_scale: 1.,
    //     substeps: 1,
    // };
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(true))
        .insert_resource(rapier_config);

    #[cfg(debug_assertions)]
    {
        app.add_plugins(LogDiagnosticsPlugin::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(WorldInspectorPlugin::new());
    }

    app.add_plugins((BlenvyPlugin::default(), BlenvyExtensions));

    app.add_plugins(NoCameraPlayerPlugin);
    app.add_plugins(BaseballFlightPlugin {
        ssw_on: true,
        magnus_on: true,
        drag_on: true,
    });
    app.add_plugins(ThirdPersonCameraPlugin);

    app.add_plugins(GameScenePlugin);

    app.run();
}

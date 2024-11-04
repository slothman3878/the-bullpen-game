mod batter;
mod errors;
mod materials;
mod pitcher;
mod prelude;
mod scenes;
mod strikezone;

use crate::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy_third_person_camera::ThirdPersonCameraPlugin;

// const WINDOW_WIDTH: f32 = 1920.0;
// const WINDOW_HEIGHT: f32 = 1024.0;

fn main() {
    let mut app = App::new();

    // app.add_plugins(DefaultPlugins.set(WindowPlugin {
    //     primary_window: Some(Window {
    //         title: "The Bullpen".to_string(),
    //         resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
    //         resizable: false,
    //         cursor: Cursor {
    //             grab_mode: CursorGrabMode::Locked,
    //             ..default()
    //         },
    //         ..Default::default()
    //     }),
    //     ..Default::default()
    // }));
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics on web build on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "The Bullpen".to_string(),
                    canvas: Some("#bevy".to_string()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
    );

    app.add_plugins(MaterialPlugin::<LineMaterial>::default());

    app.add_plugins(EguiPlugin);

    app.insert_resource(Time::<Fixed>::from_hz(60.0));

    let mut rapier_config = RapierConfiguration::new(1.);
    rapier_config.timestep_mode = TimestepMode::Fixed {
        dt: 1. / 60.,
        substeps: 100,
    };
    // rapier_config.timestep_mode = TimestepMode::Variable {
    //     max_dt: 1.0 / 60.0,
    //     time_scale: 1.,
    //     substeps: 100,
    // };
    // rapier_config.timestep_mode = TimestepMode::Interpolated {
    //     dt: 1. / 60.,
    //     substeps: 100,
    //     time_scale: 1.,
    // };

    // let mut rapier_context = RapierContext::default();
    // rapier_context.integration_parameters.max_ccd_substeps = 100;

    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(true))
        // .insert_resource(rapier_context)
        .insert_resource(rapier_config);

    #[cfg(debug_assertions)]
    {
        app.add_plugins(LogDiagnosticsPlugin::default())
            .add_plugins(RapierDebugRenderPlugin::default());

        // app.add_systems(Update, display_events);
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

fn _display_events(mut collision_events: EventReader<CollisionEvent>) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
    }
}

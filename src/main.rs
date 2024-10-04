mod blenvy_extensions;
mod prelude;

use crate::prelude::*;

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
        dt: 1. / 1000.,
        substeps: 1,
    };
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(true))
        .insert_resource(rapier_config);

    #[cfg(debug_assertions)]
    {
        app.add_plugins(LogDiagnosticsPlugin::default())
            .add_plugins(RapierDebugRenderPlugin::default());
    }

    app.add_plugins(SickleUiPlugin);
    app.add_plugins((BlenvyPlugin::default(), BlenvyExtensions));

    app.add_plugins(NoCameraPlayerPlugin);
    app.add_plugins(BaseballFlightPlugin {
        ssw_on: true,
        magnus_on: true,
        drag_on: true,
    });

    // app.add_systems(PostStartup, setup);
    app.add_systems(PostStartup, (setup_scene, spawn_camera.after(setup_scene)));

    app.run();
}

fn setup(mut commands: Commands) {
    // The main camera which will render UI
    let main_camera = commands
        .spawn((Camera3dBundle {
            camera: Camera {
                order: 1,
                clear_color: Color::BLACK.into(),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 30., 0.))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },))
        .id();

    // Use the UI builder with plain bundles and direct setting of bundle props
    let mut root_entity = Entity::PLACEHOLDER;
    commands.ui_builder(UiRoot).container(
        (
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            TargetCamera(main_camera),
        ),
        |container| {
            root_entity = container
                .spawn((NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                },))
                .id();
        },
    );

    commands.ui_builder(root_entity).spawn(NodeBundle {
        style: Style {
            width: Val::Px(200.),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        background_color: Color::srgb(0.65, 0.65, 0.65).into(),
        ..default()
    });

    commands.ui_builder(root_entity).column(|column| {
        column
            .style()
            .height(Val::Percent(100.))
            .width(Val::Percent(100.))
            .background_color(Color::srgb(0.15, 0.155, 0.16));

        // column.spawn(NodeBundle {
        //     style: Style {
        //         width: Val::Px(200.),
        //         border: UiRect::all(Val::Px(2.)),
        //         ..default()
        //     },
        //     background_color: Color::srgb(0.65, 0.65, 0.65).into(),
        //     ..default()
        // });

        column.menu_bar(|bar| {
            bar.menu(
                MenuConfig {
                    name: "Showcase".into(),
                    alt_code: KeyCode::KeyS.into(),
                    ..default()
                },
                |menu| {
                    menu.menu_item(MenuItemConfig {
                        name: "Layout".into(),
                        shortcut: vec![KeyCode::KeyL].into(),
                        alt_code: KeyCode::KeyL.into(),
                        ..default()
                    });
                    menu.menu_item(MenuItemConfig {
                        name: "Interactions".into(),
                        shortcut: vec![KeyCode::ControlLeft, KeyCode::KeyI].into(),
                        alt_code: KeyCode::KeyI.into(),
                        ..default()
                    });

                    menu.separator();

                    let icons = ThemeData::default().icons;
                    menu.menu_item(MenuItemConfig {
                        name: "Exit".into(),
                        leading_icon: icons.exit_to_app,
                        ..default()
                    });
                },
            );
        });
    });

    commands.ui_builder(root_entity).row(|row| {
        row.sized_zone(
            SizedZoneConfig {
                size: 100.,
                ..default()
            },
            |zone| {
                zone.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::srgb(0.65, 0.65, 0.65).into(),
                    ..default()
                });
            },
        );
    });
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

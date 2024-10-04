use std::f32::consts::PI;

use bevy::{
    diagnostic::LogDiagnosticsPlugin, input::common_conditions::input_just_released, math::DVec3,
    pbr::CascadeShadowConfigBuilder, prelude::*, window::WindowResolution,
};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_rapier3d::prelude::*;
use bevy_rapier_baseball_flight::prelude::*;
use blenvy::*;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1024.0;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bullpen".to_string(),
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.insert_resource(Time::<Fixed>::from_hz(60.0));

    let mut rapier_config = RapierConfiguration::new(1.);
    // rapier_config.timestep_mode = TimestepMode::Fixed {
    //     dt: 1. / 60.,
    //     substeps: 1,
    // };
    rapier_config.timestep_mode = TimestepMode::Variable {
        max_dt: 1.0 / 60.0,
        time_scale: 1.,
        substeps: 1,
    };

    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(true))
        .insert_resource(rapier_config);
    #[cfg(debug_assertions)]
    {
        app.add_plugins(LogDiagnosticsPlugin::default())
            .add_plugins(RapierDebugRenderPlugin::default());
    }

    app.add_plugins(BlenvyPlugin::default());
    app.add_plugins(NoCameraPlayerPlugin);
    app.add_plugins(BaseballFlightPlugin {
        ssw_on: true,
        magnus_on: true,
        drag_on: true,
    });

    app.add_systems(PostStartup, (setup_scene, spawn_camera.after(setup_scene)));

    app.add_systems(
        Update,
        spawn_ball.run_if(input_just_released(KeyCode::KeyR)),
    );

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
            transform: Transform::from_xyz(-0.0, 5.0, -0.)
                .looking_at(Vec3::new(0., 1.2, 0.), Vec3::Y),
            ..default()
        },
    ));
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    // ground plane
    commands
        .spawn((
            RigidBody::Fixed,
            Collider::cuboid(30., 0.2, 30.),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., -0.2, 0.))),
            InheritedVisibility::VISIBLE,
        ))
        .with_children(|child| {
            child.spawn((PbrBundle {
                mesh: meshes.add(Plane3d::default().mesh().size(60.0, 60.0)),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
                ..default()
            },));
        });

    commands.spawn((PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(60.0, 60.0)),
        transform: Transform::from_rotation(Quat::from_rotation_z(PI)),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    },));

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // commands.spawn((
    //     BlueprintInfo::from_path("levels/Bullpen.glb"),
    //     SpawnBlueprint,
    //     HideUntilReady,
    //     GameWorldTag,
    // ));
}

fn spawn_ball(
    mut commands: Commands,
    mut ev_activate_aerodynamics: EventWriter<ActivateAerodynamicsEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let gyro_pole = GyroPole::default();
    let spin_efficiency: f32 = 1.0;
    let velocity: f32 = 96. * MPH_TO_FTS;
    let spin_rate: f32 = 2400.;
    let seam_z_angle: f32 = PI / 2.;
    let tilt = Tilt::from_hour_mintes(12, 0);

    let fixed_spin_rate = if spin_rate == 0. { 1. } else { spin_rate };

    let gyro = match gyro_pole {
        GyroPole::Left => spin_efficiency.asin(),
        GyroPole::Right => PI - spin_efficiency.asin(),
    };

    let spin_x_0 = fixed_spin_rate * (spin_efficiency * tilt.get().sin());
    let spin_y_0 = fixed_spin_rate * gyro.cos(); // ((1. - spin_efficiency.powi(2)).sqrt());
    let spin_z_0 = -fixed_spin_rate * (spin_efficiency * tilt.get().cos());
    let spin = Vec3::new(
        spin_x_0 * RPM_TO_RADS,
        spin_y_0 * RPM_TO_RADS, // - RPM_TO_RAD ???
        spin_z_0 * RPM_TO_RADS,
    );

    // let entity = commands
    //     .spawn((
    //         Name::new("ball"),
    //         //
    //         BaseballFlightBundle::default(),
    //         //
    //         ExternalForce::default(),
    //         TransformBundle::from_transform(Transform::from_translation(Vec3::new(
    //             0.48, 1.82, 16.764,
    //         ))),
    //         Velocity {
    //             linvel: (-Vec3::Y * velocity).from_baseball_coord_to_bevy(),
    //             angvel: spin.from_baseball_coord_to_bevy(),
    //         },
    //         //
    //         Restitution {
    //             coefficient: 0.546,
    //             combine_rule: CoefficientCombineRule::Min,
    //         },
    //         //
    //         InheritedVisibility::VISIBLE,
    //     ))
    //     .with_children(|child| {
    //         child.spawn((PbrBundle {
    //             mesh: meshes.add(Sphere::new(0.03).mesh().uv(32, 18)),
    //             material: materials.add(StandardMaterial {
    //                 base_color: Color::BLACK,
    //                 perceptual_roughness: 1.0,
    //                 ..default()
    //             }),
    //             ..default()
    //         },));
    //     })
    //     .id();

    ev_activate_aerodynamics.send(ActivateAerodynamicsEvent {
        // entity,
        seam_y_angle: 0.,
        seam_z_angle,
        entity: {
            commands
                .spawn((
                    Name::new("ball"),
                    //
                    BaseballFlightBundle::default(),
                    //
                    ExternalForce::default(),
                    TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                        0.48, 1.82, 16.764,
                    ))),
                    Velocity {
                        linvel: (-Vec3::Y * velocity).from_baseball_coord_to_bevy(),
                        angvel: spin.from_baseball_coord_to_bevy(),
                    },
                    //
                    Restitution {
                        coefficient: 0.546,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    //
                    InheritedVisibility::VISIBLE,
                ))
                .with_children(|child| {
                    child.spawn((PbrBundle {
                        mesh: meshes.add(Sphere::new(0.03).mesh().uv(32, 18)),
                        material: materials.add(StandardMaterial {
                            base_color: Color::BLACK,
                            perceptual_roughness: 1.0,
                            ..default()
                        }),
                        ..default()
                    },));
                })
                .id()
        },
    });
}

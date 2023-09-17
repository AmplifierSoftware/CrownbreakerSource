use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;


mod build_utils;
mod pop_utils;
fn main() {
   
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Crown Breaker v:predev".to_string(),
                    
                    ..Default::default()
            }),
            ..Default::default()
        })
        )
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(pop_utils::pops::PopPlugin)
        .add_plugin(build_utils::buildtest::BuildPlugin)
        .add_plugin(build_utils::CruveGeneratorPlugin)
        //.add_plugin(build_utils::curve::CurveCalculatorPlugin)
        .add_startup_system(start)

        .run();
}
fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(100.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    }, PickableBundle::default(),RaycastPickTarget::default()));
    // Cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // Camera
    commands.spawn((
        Camera3dBundle::default(),
        RaycastPickCamera::default(), 
        PanOrbitCamera{
            button_orbit:MouseButton::Right,
            button_pan:MouseButton::Middle,
            ..default()
        } // Enable picking with this camera
    ));
}
    


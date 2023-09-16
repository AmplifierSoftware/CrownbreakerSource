use belly::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod pop_utils;

use pop_utils::pops;


fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(BellyPlugin)
        .add_plugin(pops::PopPlugin)
        .add_startup_system(test_sys)
        
        .run();
}
fn test_sys(mut commands: Commands,    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,){
    commands.spawn(Camera3dBundle::default());
    commands.add(StyleSheet::load("stylesheet.ess"));
    commands.add(eml! {
        <div>
            <div c:container>
                <button>"test"</button>
                <button>"test2"</button>
                <button>"test3"</button>
                <button>"test4"</button>
            </div>
        </div>
    });
    let sphere_mesh = meshes.add(
        Mesh::try_from(shape::Icosphere {
            radius: 0.45,
            ..default()
        })
        .unwrap(),
    );
    commands.spawn(PbrBundle {
        mesh: sphere_mesh,
        material: materials.add(StandardMaterial {
            base_color: Color::hex("#ffd891").unwrap(),
            unlit: true,
            ..default()
        }), ..default()

    });
}

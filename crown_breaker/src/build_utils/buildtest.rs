use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy::window::*;

use crate::*;
pub struct BuildPlugin;

struct MousePosition(Vec3);

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(build_sys);
    }
}

fn build_sys(mut commands: Commands, asset: Res<AssetServer>, selection: Query<(Entity, &PickSelection)>,
mouse_input: Res<Input<MouseButton>>,mut mouse_position: ResMut<MousePosition>,) {
    

    if mouse_input.just_pressed(MouseButton::Left) {
       for (entity,selection) in &selection {
        if selection.is_selected {
            spawn_brick(&mut commands,&asset, window_q.single().cursor_position);
            println!("spawned brick");
        }
    }
}

fn spawn_brick(commands: &mut Commands, assets: &AssetServer , position: Vec3) -> Entity {
    let brick = assets.load("Models/brick.glb#Scene0");
    commands
        .spawn(SpatialBundle::from_transform(Transform::from_translation(position)))
        .insert(Name::new("Brick"))
        .with_children(|commands| {
            commands.spawn(SceneBundle{
                scene: brick.clone(),
                transform: Transform::from_xyz(position.x,position.y,position.z),
                ..Default::default()
            });

        })
        .id()
    }
}

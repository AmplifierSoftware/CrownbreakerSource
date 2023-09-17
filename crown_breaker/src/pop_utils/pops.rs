use bevy::prelude::*;

pub struct PopPlugin;

impl Plugin for PopPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_pop);
    
    }
}
#[derive(Component)]
struct Age(i32);



#[derive(Bundle)]
pub struct Pop {
    name: Name,
    age: Age,
}

fn generate_pop(mut commands: Commands) {
    let v = 3;

    for i in 0..v {
        commands.spawn(Pop{
            name: Name::new(format!("Pop {}",i)),
            age: Age(80),
        });
    }
}

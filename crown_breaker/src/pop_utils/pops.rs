use bevy::prelude::*;


enum RaceType{
    human,
    elf,
    dwarf,
    ork,
}

#[derive(Component)]
struct Pop{
    pub name: String,
}

fn generate_pop(mut commands: Commands){
    let ras = 3;
    for i in 0..ras{
       commands.spawn(Pop{
        name: "test".to_string(),
       });
    }  
}
fn list_pops(query: Query<&Pop>)
{

}
pub struct PopPlugin;

impl Plugin for PopPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_pop)
            .add_startup_system(list_pops);
    }
}
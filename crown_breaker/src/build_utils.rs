use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub mod buildtest;
pub mod curve;



fn curve_generator_ui(mut contexts: EguiContexts) {
    egui::Window::new("Point Generator").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
                let mut brick_length = 0.3;
                let mut wall_length = 10.0;
                let mut points = 0; 
            ui.add(egui::Slider::new(&mut brick_length, 0.0..=10.0).text("Brick Length"));
        
        
            ui.add(egui::Slider::new(&mut wall_length, 1.0..=10.0).text("Wall Length"));
        
        
            if ui.button("Generate Points").clicked() {
                let pointnum = curve::generate_points(wall_length, brick_length, &mut points);
                println!("WallPoints: {}", pointnum);
            }
        }); 
        
    });
}

pub struct CruveGeneratorPlugin;

impl Plugin for CruveGeneratorPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(curve_generator_ui);
    }
}
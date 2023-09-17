use bevy::prelude::*;

#[derive(Default)]
pub struct CurveS {
    points: Vec<Vec3>,
    brick_length: f32,
    points_u: u32,
    wall_length: f32,
}



pub fn generate_points(wall: f32, brl: f32, pou: &mut u32) -> u32 {
    *pou = (wall / brl) as u32;
    return *pou
}
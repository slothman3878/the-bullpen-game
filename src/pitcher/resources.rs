use crate::prelude::*;

#[derive(Debug, Resource, Clone, Copy)]
pub(crate) struct SelectedPitchParameters(pub PitchParams);

#[derive(Debug, Reflect, Copy, Clone)]
pub(crate) struct PitchParams {
    // not a parameter controlled by user
    // dependent on release moment, player stats, etc
    pub starting_point: Vec3,
    pub speed: f32,     // mph
    pub spin_rate: f32, // rpm
    pub direction: Vec3,
    // can partially be controlled by user
    // sliders, for intsance, are given less than .50 spin efficiency
    pub spin_efficiency: f32, // [0, 1]
    // dependent on arm angle
    pub tilt: Tilt, // rad
    // can be controlled by user
    // can be saved
    pub gyro_pole: GyroPole,
    pub seam_y_angle: f32,
    pub seam_z_angle: f32,
}

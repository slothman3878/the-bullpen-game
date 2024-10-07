use crate::prelude::*;

#[derive(Debug, Resource, Clone, Copy)]
pub(crate) struct SelectedPitchParameters(pub PitchParams);

#[derive(Debug, Reflect, Copy, Clone)]
pub(crate) struct PitchParams {
    // not a parameter controlled by user
    // dependent on release moment, player stats, etc
    pub starting_point: Vec3,
    pub velocity: f32,  // mph
    pub spin_rate: f32, // rpm
    // pub aim_point: Vec3,
    // can partially be controlled by user
    // sliders, for intsance, are given less than .50 spin efficiency
    pub spin_efficiency: f32, // [0, 1]
    // dependent on arm angle
    pub tilt: Tilt, // rad
    // can be controlled by user
    pub gyro_pole: GyroPole,
    pub seam_y_angle: f32,
    pub seam_z_angle: f32,
}

impl PitchParams {
    pub(crate) fn demo() -> Self {
        let gyro_pole = GyroPole::default();
        let spin_efficiency: f32 = 1.0;
        let velocity: f32 = 96. * MPH_TO_FTS;
        let spin_rate: f32 = 2400.;
        let seam_y_angle: f32 = 0.;
        let seam_z_angle: f32 = std::f32::consts::PI / 2.;
        let tilt = Tilt::from_hour_mintes(12, 0);

        Self {
            starting_point: Vec3::new(0.48, 1.82, 16.764),
            velocity,
            spin_rate,
            spin_efficiency,
            tilt,
            gyro_pole,
            seam_y_angle,
            seam_z_angle,
        }
    }
}

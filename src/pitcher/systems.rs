use core::f32;

use super::*;

pub(crate) fn start_pitch(
    // mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut selected_pitch_params: ResMut<SelectedPitchParameters>,
    camera_query: Query<&GlobalTransform, With<PitcherCameraMarker>>,
) {
    if let Ok(camera_global_transform) = camera_query.get_single() {
        let camera_transform = camera_global_transform.compute_transform();
        let ray_origin = camera_transform.translation;
        let ray_dir = camera_transform.rotation.mul_vec3(-Vec3::Z).normalize();
        let max_toi = f32::INFINITY;
        let query = QueryFilter::new();

        let direction = match rapier_context.cast_ray(ray_origin, ray_dir, max_toi, true, query) {
            Some((_entity, toi)) => {
                let aim_point = ray_origin + ray_dir * toi;
                (aim_point - ray_origin).normalize()
            }
            None => ray_dir,
        };

        selected_pitch_params.0.direction = direction;
    }
}

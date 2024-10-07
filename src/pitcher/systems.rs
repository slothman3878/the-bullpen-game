use super::components::*;
use crate::prelude::*;

pub(crate) fn start_pitch(
    // mut commands: Commands,
    rapier_context: Res<RapierContext>,
    camera_query: Query<&GlobalTransform, With<PitcherCameraMarker>>,
) {
    if let Ok(camera_global_transform) = camera_query.get_single() {
        info!("hello??");
        let camera_transform = camera_global_transform.compute_transform();
        let ray_origin = camera_transform.translation;
        let ray_dir = camera_transform.rotation.mul_vec3(-Vec3::Z);
        info!("origin {:?}, dir {:?}", ray_origin, ray_dir);
        let max_toi = 20.;

        if let Some((entity, toi)) =
            rapier_context.cast_ray(ray_origin, ray_dir, max_toi, true, QueryFilter::default())
        {
            let hit_point = ray_origin + ray_dir * toi;
            info!("Entity {:?} hit at point {}", entity, hit_point);
        }

        rapier_context.intersections_with_ray(
            ray_origin,
            ray_dir,
            max_toi,
            true,
            QueryFilter::default(),
            |entity, intersection| {
                // Callback called on each collider hit by the ray.
                let hit_point = intersection.point;
                let hit_normal = intersection.normal;
                println!(
                    "Entity {:?} hit at point {} with normal {}",
                    entity, hit_point, hit_normal
                );
                true // Return `false` instead if we want to stop searching for other hits.
            },
        );
    }
}

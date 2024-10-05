use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use blenvy::*;

#[derive(Debug)]
pub struct BlenvyExtensions;

impl Plugin for BlenvyExtensions {
    fn build(&self, app: &mut App) {
        app.register_type::<BlueprintRapierTriMeshComponent>()
            .add_systems(
                Update,
                (add_colliders).in_set(GltfBlueprintsSet::AfterSpawn),
            );
    }

    fn cleanup(&self, _app: &mut App) {
        // TODO:
    }
}

#[derive(Debug, Component, Clone, Reflect)]
#[reflect(Component)]
pub struct BlueprintRapierTriMeshComponent;

fn add_colliders(
    mut commands: Commands,
    scene_meshes: Query<(Entity, &BlueprintRapierTriMeshComponent, &Handle<Mesh>), Added<Name>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    // iterate over all meshes in the scene and match them by their name.
    for (entity, _, mesh_handle) in scene_meshes.iter() {
        let mesh = meshes.get(mesh_handle).unwrap();
        let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
        commands.entity(entity).insert(collider);
        //
        info!("removing rapier trimesh marker from component");
        commands
            .entity(entity)
            .remove::<BlueprintRapierTriMeshComponent>();
    }
}

#[derive(Debug, Component, Clone, Reflect)]
#[reflect(Component)]
pub struct BlueprintRapierTriMeshComponent;

pub fn add_colliders(
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

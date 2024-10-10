use super::*;

pub(crate) fn spawn_arms(mut commands: Commands) {
    let parent_entity = commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .id();

    let x = Vec3::X;
    let joint = RevoluteJointBuilder::new(x)
        .local_anchor1(Vec3::new(0.0, 0.0, 1.0))
        .local_anchor2(Vec3::new(0.0, 0.0, -3.0));
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(ImpulseJoint::new(parent_entity, joint));
}

use bevy::prelude::{Camera3d, Commands, Component, Entity, PerspectiveProjection, Projection, Query, Transform, Vec3, With, Without};
use bevy::utils::default;
use shared::plugins::combatant::PlayerCombatant;

#[derive(Component)]
pub struct CameraAttached;

#[derive(Component)]
#[allow(dead_code)]
pub struct CombatantCamera{
    distance: f32,
    height: f32,
    offset_side: f32
}

pub fn create_combatant_camera(
    mut commands: Commands,
    character_query: Query<Entity,(With<PlayerCombatant>,Without<CameraAttached>)>
){
    for entity in character_query.iter() {
        commands.entity(entity).insert(CameraAttached);
        
        commands.spawn((
            Camera3d::default(),
            Transform::default(),
            CombatantCamera{
                distance: 3.0,
                height: 0.4,
                offset_side: 0.6,
            },
            Projection::from(PerspectiveProjection{
                fov: 70.0_f32.to_radians(),
                ..default()
            }),
        ));
    }
}

pub fn update_combatant_camera_transform(
    character_query: Query<&Transform, With<CameraAttached>>,
    mut camera_query: Query<(&mut Transform, &CombatantCamera), (With<CombatantCamera>, Without<CameraAttached>)>
){
    for character_transform in character_query.iter() {
        for (mut camera_transform,combatant_camera) in camera_query.iter_mut() {
            let current_translation = character_transform.translation;
            let forward_vector = character_transform.forward().as_vec3();
            let left_vector = character_transform.left().as_vec3();
            let new_camera_translation = (current_translation + (forward_vector * combatant_camera.distance) + (left_vector * combatant_camera.offset_side)) + Vec3::new(0.0,combatant_camera.height,0.0);

            camera_transform.translation = new_camera_translation;
            camera_transform.look_at(new_camera_translation - forward_vector,Vec3::Y);
        }
    }
}
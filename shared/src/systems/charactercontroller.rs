use avian3d::prelude::{Collider, ComputedMass, ExternalForce, Gravity, LayerMask, LinearVelocity, ShapeCastConfig, ShapeHitData, SpatialQuery, SpatialQueryFilter};
use bevy::ecs::entity::EntityHashSet;
use bevy::math::{vec3, Dir3};
use bevy::prelude::{Entity, Fixed, Quat, Query, Res, Time, Transform, Vec3, With};
use crate::{GameMask, InteractNetworkAble};
use crate::plugins::combatant::{CharacterController};
use crate::plugins::statesmachine::{CurrentStates, States, StatesValues};

const FLOAT_DISTANCE: f32 = 0.1;

pub fn find_ground(
    entity: Entity,
    query: &SpatialQuery,
    translation: &Vec3,
    rotation: &Quat,
    collider: &Collider
)-> Option<ShapeHitData> {
    let capsule_collider= if let Some(capsule) = collider.shape().as_capsule() {capsule} else {return None};
    let height = capsule_collider.height() + (capsule_collider.radius * 2.0);
    let mut ignore_list = EntityHashSet::default();

    ignore_list.insert(entity);

    query.cast_shape(collider,*translation + vec3(0.0,height,0.0),*rotation,-Dir3::Y,&ShapeCastConfig{
        max_distance: height * 1.1,
        target_distance: 0.0,
        compute_contact_on_penetration: true,
        ignore_origin_penetration: true
    },&SpatialQueryFilter{
        mask: LayerMask::from(GameMask::Default),
        excluded_entities: ignore_list,
    })
}

pub fn check_is_grounded(
    query: SpatialQuery,
    mut character_query: Query<(Entity, &mut CharacterController, &Collider, &Transform), (With<InteractNetworkAble>, With<CharacterController>)>
){
    for (entity, mut character_controller, collider, transform) in character_query.iter_mut(){
        character_controller.shape_hit_data = find_ground(entity,&query,&transform.translation,&transform.rotation,&collider);
        character_controller.grounded = if character_controller.shape_hit_data.is_some() {true} else {false};
    }
}

pub fn adjust_collider_float(
    mut character_query: Query<(&CharacterController, &Collider, &Transform, &mut ExternalForce, &mut LinearVelocity, &ComputedMass), (With<InteractNetworkAble>,With<CharacterController>)>
){
    for (character_controller, collider, transform, mut external_forces, mut linear_velocity, computed_mass) in character_query.iter_mut(){
        let capsule_collider= if let Some(capsule) = collider.shape().as_capsule() {capsule} else {continue};
        let height: f32 = capsule_collider.height() + (capsule_collider.radius * 2.0);
        let half_height = height / 2.0;
        let current_translation = transform.translation;
        let mass_value = computed_mass.value();

        if let Some(ref shape_hit_data) = character_controller.shape_hit_data{
            let ground_point = shape_hit_data.point1;
            let float_point = (ground_point.y + half_height) + FLOAT_DISTANCE;
            let stand_difference = current_translation.y - float_point;

            if stand_difference.abs() <= 0.005 {
                if linear_velocity.y != 0.0 {
                    linear_velocity.y = 0.0;
                }

                if external_forces.y != 0.0 {
                    external_forces.y = 0.0;
                }
            }else if stand_difference > 0.0 {
                external_forces.apply_force(Vec3::new(0.0, stand_difference * mass_value, 0.0));
            }else {
                external_forces.apply_force(Vec3::new(0.0, -stand_difference * mass_value, 0.0));
            }
        }
    }
}

pub fn character_walk(
    mut character_query: Query<(&mut CurrentStates, &mut LinearVelocity), (With<CharacterController>, With<InteractNetworkAble>)>
){
    for (mut current_states, mut linear_velocity) in character_query.iter_mut(){
        if !current_states.0.contains_key(&States::Walking){
            return;
        }

        if let Some(StatesValues::Walking(walking_direction)) = current_states.0.get_mut(&States::Walking).unwrap().values{
            linear_velocity.z = walking_direction.z;
        }
    }
}

pub fn control_gravity(
    mut character_query: Query<(&mut CurrentStates, &mut LinearVelocity), (With<CharacterController>, With<InteractNetworkAble>)>,
    gravity: Res<Gravity>,
    time_fixed: Res<Time<Fixed>>,
){
    let gravity_force = gravity.0.y * time_fixed.delta().as_secs_f32();

    for (_current_states, mut linear_velocity) in character_query.iter_mut(){
        if linear_velocity.x != 0.0 {
            linear_velocity.x = if linear_velocity.x > 0.0 { (linear_velocity.x + gravity_force).max(0.0) } else {(linear_velocity.x - gravity_force).min(0.0)};
        }

        if linear_velocity.z != 0.0 {
            linear_velocity.z = if linear_velocity.z > 0.0 { (linear_velocity.z + gravity_force).max(0.0) } else {(linear_velocity.z - gravity_force).min(0.0)};
        }
    }
}
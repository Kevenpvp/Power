use bevy::math::Vec3;
use bevy::prelude::{default, Vec2};
use crate::plugins::statesmachine::{CurrentStates, StateInfos, States, StatesValues};

pub fn move_action(
    move_dir: Vec2,
    current_states: &mut CurrentStates
){
    if move_dir.y > 0.0 || move_dir.x > 0.0 {
        current_states.transition(&States::Walking,StateInfos{
            values: Some(StatesValues::Walking(Vec3::new(-move_dir.x,0.0,move_dir.y))),
            ..default()
        });
    }else {
        current_states.transition(&States::Idle,StateInfos{
            values: None,
            ..default()
        });
    }
}
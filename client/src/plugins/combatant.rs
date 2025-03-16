use bevy::app::App;
use bevy::prelude::{FixedUpdate, IntoSystemConfigs, Plugin, PostUpdate, Query, Res, TransformSystem, With};
use leafwing_input_manager::prelude::ActionState;
use lightyear::inputs::leafwing::input_buffer::InputBuffer;
use lightyear::prelude::client::Rollback;
use lightyear::prelude::TickManager;
use shared::InteractNetworkAble;
use shared::plugins::combatant::PlayerCombatant;
use shared::plugins::statesmachine::{CurrentStates, StatesApplied};
use shared::protocol::CharacterAction;
use shared::systems::characteractions::move_action;
use shared::systems::charactercontroller::check_is_grounded;
use crate::systems::camera::{create_combatant_camera,update_combatant_camera_transform};
use crate::systems::states::{check_idle_state, check_walking_state};

pub struct CombatantPlugin;

impl Plugin for CombatantPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate,(check_idle_state,check_walking_state).before(check_is_grounded));
        app.add_systems(PostUpdate,(create_combatant_camera,update_combatant_camera_transform,handle_combatant_actions).chain().before(TransformSystem::TransformPropagate));
    }
}

pub fn handle_combatant_actions(
    mut query: Query<(&ActionState<CharacterAction>, &InputBuffer<CharacterAction>, &mut CurrentStates),(With<InteractNetworkAble>, With<PlayerCombatant>, With<StatesApplied>)>,
    tick_manager: Res<TickManager>,
    rollback: Option<Res<Rollback>>,
){
    let tick = rollback
        .as_ref()
        .map(|rb| tick_manager.tick_or_rollback_tick(rb))
        .unwrap_or(tick_manager.tick());

    for (action_state, input_buffer, mut current_states) in query.iter_mut() {
        let action_state_correctly = if input_buffer.get(tick).is_some() {action_state} else {
            if let Some((_, prev_action_state)) = input_buffer.get_last_with_tick() {prev_action_state} else {action_state}
        };

        let move_dir = action_state_correctly
            .axis_pair(&CharacterAction::Move)
            .clamp_length_max(1.0);

        move_action(move_dir, &mut current_states);
    }
}
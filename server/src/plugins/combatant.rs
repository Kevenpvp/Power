use bevy::app::{App, PostUpdate};
use bevy::prelude::{IntoSystemConfigs, Plugin, Query, TransformSystem, With};
use leafwing_input_manager::action_state::ActionState;
use shared::InteractNetworkAble;
use shared::plugins::statesmachine::{CurrentStates, StatesApplied};
use shared::protocol::CharacterAction;
use shared::systems::characteractions::move_action;

pub struct CombatantPlugin;

impl Plugin for CombatantPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate,handle_combatant_actions.before(TransformSystem::TransformPropagate));
    }
}

pub fn handle_combatant_actions(
    mut query: Query<(&ActionState<CharacterAction>, &mut CurrentStates),(With<InteractNetworkAble>, With<StatesApplied>)>,
){
    for (action_state, mut current_states) in &mut query {
        let move_dir = action_state
            .axis_pair(&CharacterAction::Move)
            .clamp_length_max(1.0);

        move_action(move_dir, &mut current_states);
    }
}
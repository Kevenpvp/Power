use bevy::prelude::{Entity, EventReader, EventWriter, Query, With};
use shared::InteractNetworkAble;
use shared::plugins::combatant::CombatantMarker;
use shared::plugins::statesmachine::{StateAdded, States, StatesApplied};
use crate::plugins::animations::{AnimationsLoaded, PlayAnimation};

pub fn check_idle_state(
    mut event_state_added: EventReader<StateAdded>,
    mut event_play_animation: EventWriter<PlayAnimation>,
    mut character_query: Query<(Entity, &mut StatesApplied, Option<&AnimationsLoaded>), (With<CombatantMarker>, With<InteractNetworkAble>)>
){
    for event in event_state_added.read(){
        if event.1 == States::Idle {
            for (entity, mut states_applied, animations_loaded) in character_query.iter_mut(){
                if animations_loaded.is_none(){
                    states_applied.failed_apply(&event.1);
                    continue;
                }

                event_play_animation.send(PlayAnimation(entity,"Idle".to_string()));
            }
        }
    }
}

pub fn check_walking_state(
    mut event_state_added: EventReader<StateAdded>,
    mut event_play_animation: EventWriter<PlayAnimation>,
    mut character_query: Query<(Entity, &mut StatesApplied, Option<&AnimationsLoaded>), (With<CombatantMarker>, With<InteractNetworkAble>)>
){
    for event in event_state_added.read(){
        if event.1 == States::Walking {
            for (entity, mut states_applied, animations_loaded) in character_query.iter_mut(){
                if animations_loaded.is_none(){
                    states_applied.failed_apply(&event.1);
                    continue;
                }

                event_play_animation.send(PlayAnimation(entity,"Walking".to_string()));
            }
        }
    }
}

use std::time::{SystemTime, UNIX_EPOCH};
use bevy::app::{App, FixedPreUpdate};
use bevy::math::Vec3;
use bevy::prelude::{Added, Changed, Commands, Component, Entity, Event, EventWriter, IntoSystemConfigs, Or, Plugin, Query, Reflect, With, Without};
use bevy::utils::hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use crate::{InteractNetworkAble};

pub struct StatesMachinePlugin;

#[allow(dead_code)]
pub struct StatesSettings{
    blacklist: Vec<States>,
    stop_list: Vec<States>,
    stop_all: bool
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub enum StatesValues{
    Walking(Vec3)
}

#[derive(Event)]
#[allow(dead_code)]
pub struct StateAdded(pub Entity,pub States);

#[derive(Event)]
#[allow(dead_code)]
pub struct StateRemoved(pub Entity,pub States);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct StateInfos{
    pub start: Option<u128>,
    pub duration: f32,
    pub in_cooldown: bool,
    pub stopped_states: Vec<States>,
    pub values: Option<StatesValues>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Reflect, Eq, Hash)]
pub enum States{
    Idle,
    Walking,
    Jumping,
    Died
}

#[derive(Component)]
pub struct StatesApplied(pub Vec<States>);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct CurrentStates(pub HashMap<States,StateInfos>);

impl Plugin for StatesMachinePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CurrentStates>();
        app.add_event::<StateAdded>();
        app.add_event::<StateRemoved>();
        app.add_systems(FixedPreUpdate,(current_states_added,check_states_changed,check_states_failed_apply).chain());
    }
}

impl Default for StateInfos {
    fn default() -> Self {
        Self {
            start: match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration_since) => {Some(duration_since.as_millis())},
                Err(_) => {None}
            },
            duration: 0.0,
            in_cooldown: false,
            stopped_states: Vec::new(),
            values: None
        }
    }
}

impl States {
    pub fn get_settings(&self) -> StatesSettings{
        match self {
            States::Idle => {
                StatesSettings {
                    blacklist: vec![States::Died,States::Jumping],
                    stop_list: vec![States::Walking],
                    stop_all: false
                }
            },
            States::Walking => {
                StatesSettings {
                    blacklist: vec![States::Died,States::Jumping],
                    stop_list: vec![States::Idle],
                    stop_all: false
                }
            }
            States::Jumping => {
                StatesSettings {
                    blacklist: vec![States::Died],
                    stop_list: vec![States::Idle,States::Walking],
                    stop_all: false
                }
            },
            States::Died => {
                StatesSettings {
                    blacklist: vec![],
                    stop_list: vec![],
                    stop_all: true
                }
            }
        }
    }
}

impl StatesApplied {
    pub fn failed_apply(&mut self, failed_apply_state: &States){
        self.0.retain(|state| state != failed_apply_state);
    }
}

impl Default for CurrentStates {
    fn default() -> Self {
        Self(HashMap::from([
            (States::Idle,StateInfos::default())
        ]))
    }
}

impl CurrentStates {
    pub fn can_transition(&self, transition_state: &States) -> bool{
        let mut valid_transition = true;
        let settings = transition_state.get_settings();
        let current_states = &self.0;

        if current_states.contains_key(transition_state){
            valid_transition = false
        }else {
            for state in settings.blacklist.iter() {
                if current_states.contains_key(state){
                    valid_transition = false;
                    break;
                }
            };
        }

        valid_transition
    }

    pub fn transition(&mut self, transition_state: &States, mut state_infos: StateInfos){
        if !self.can_transition(transition_state){
            return;
        }

        let settings = transition_state.get_settings();
        let mut stopped_states: Vec<States> = Vec::new();
        let current_states = &mut self.0;

        for state in settings.stop_list.iter() {
            stopped_states.push(state.clone());
            current_states.remove(state);
        }

        state_infos.stopped_states = stopped_states;
        current_states.insert(transition_state.clone(), state_infos);
    }
}

pub fn current_states_added(
    mut commands: Commands,
    query: Query<(Entity, &CurrentStates), (Without<StatesApplied>, With<InteractNetworkAble>)>
) {
    for (entity, _) in query.iter() {
        commands.entity(entity).insert(StatesApplied(Vec::new()));
    }
}

fn check_states_changed(
    mut event_state_added: EventWriter<StateAdded>,
    mut event_state_removed: EventWriter<StateRemoved>,
    mut query: Query<(Entity, &CurrentStates, &mut StatesApplied), (Or<(Changed<CurrentStates>, Added<CurrentStates>)>, With<InteractNetworkAble>, With<CurrentStates>, With<StatesApplied>)>,
){
    for (entity,current_states, mut states_applied) in query.iter_mut() {
        let current_states_map = &current_states.0;
        let states_applied_list = &mut states_applied.0;
        let mut states_removed_list: Vec<usize> = Vec::new();
        let mut index_state = 0;

        for (state,_) in current_states_map.iter() {
            if !states_applied_list.contains(state) {
                states_applied_list.push(state.clone());
                event_state_added.send(StateAdded(entity, state.clone()));
            }
        }

        for state in states_applied_list.iter() {
            if !current_states_map.contains_key(state) {
                states_removed_list.push(index_state);
                event_state_removed.send(StateRemoved(entity, state.clone()));
            }

            index_state += 1
        }

        for index in states_removed_list {
            states_applied_list.remove(index);
        }
    }
}

fn check_states_failed_apply(
    mut event_state_added: EventWriter<StateAdded>,
    mut query: Query<(Entity, &CurrentStates, &mut StatesApplied), (Changed<StatesApplied>, With<StatesApplied>,With<InteractNetworkAble>,Changed<StatesApplied>)>
){
    for (entity,current_states, mut states_applied) in query.iter_mut() {
        let current_states_map = &current_states.0;
        let states_applied_list = &mut states_applied.0;

        for (state,_) in current_states_map.iter() {
            if !states_applied_list.contains(state) {
                states_applied_list.push(state.clone());
                event_state_added.send(StateAdded(entity, state.clone()));
            }
        }
    }
}

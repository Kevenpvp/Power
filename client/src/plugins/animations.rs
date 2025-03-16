use std::time::Duration;
use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::gltf::GltfAssetLabel;
use bevy::prelude::{Added, AnimationGraph, AnimationGraphHandle, AnimationNodeIndex, AnimationPlayer, AnimationTransitions, Assets, Commands, Component, Entity, Event, EventReader, Parent, PostUpdate, Query, Res, ResMut, Update};
use bevy::utils::hashbrown::HashMap;

pub struct AnimationPlugin;

#[derive(Event)]
#[allow(dead_code)]
pub struct PlayAnimation(pub Entity,pub String);

#[derive(Component)]
pub struct AnimationsLoaded;

#[derive(Component)]
#[allow(dead_code)]
pub struct AnimationComponent{
    ancestor_entity: Entity,
    animations_names: HashMap<String,AnimationNodeIndex>,
    node_indices: Vec<AnimationNodeIndex>
}

impl Plugin for AnimationPlugin{
    fn build(&self, app: &mut App) {
        app.add_event::<PlayAnimation>();
        app.add_systems(PostUpdate,create_animation_component);
        app.add_systems(Update,play_animation);
    }
}

fn create_animation_component(
    mut commands: Commands,
    character_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
){
    for entity in character_query.iter(){
        let mut current_entity = entity;

        while let Ok(parent) = parent_query.get(current_entity) {
            current_entity = parent.get();
        }
        
        let (graph, node_indices) = AnimationGraph::from_clips([
            asset_server.load(GltfAssetLabel::Animation(0).from_asset("animations/Idle.glb")),
            asset_server.load(GltfAssetLabel::Animation(0).from_asset("animations/Walking.glb")),
        ]);

        commands.entity(entity).insert((AnimationComponent{
            ancestor_entity: current_entity,
            animations_names: HashMap::from([
                ("Idle".to_string(),node_indices[0]),
                ("Walking".to_string(),node_indices[1]),
            ]),
            node_indices
        },AnimationTransitions::new(),AnimationGraphHandle(graphs.add(graph))));

        commands.entity(current_entity).insert(AnimationsLoaded);
    }
}

pub fn play_animation(
    mut play_animation_event: EventReader<PlayAnimation>,
    mut character_query: Query<(&mut AnimationPlayer, &mut AnimationTransitions, &mut AnimationComponent)>,
){
    for event in play_animation_event.read() {
        for (mut animation_player, mut transitions, animations_component) in character_query.iter_mut(){
            if animations_component.ancestor_entity != event.0 {
                continue
            }

            transitions.play(&mut animation_player,animations_component.animations_names[&event.1],Duration::ZERO).repeat();
        }
    }
}
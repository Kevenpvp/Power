pub mod plugins;
pub mod systems;

use avian3d::collision::Collider;
use avian3d::prelude::RigidBody;
use bevy::asset::Assets;
use bevy::DefaultPlugins;
use bevy::pbr::{PointLight, StandardMaterial};
use bevy::prelude::{default, Added, App, Color, Commands, Cylinder, Entity, First, Mesh, Mesh3d, MeshMaterial3d, Query, ResMut, Startup, Transform, With};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use lightyear::prelude::Replicated;
use shared::GameMask;
use shared::protocol::FloorMarker;
use crate::plugins::animations::AnimationPlugin;
use crate::plugins::combatant::CombatantPlugin;
use crate::plugins::connection::ClientPlugin;

fn default_stuff(
    mut commands: Commands,
){
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn floor_load(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    floor_query: Query<Entity,(Added<Replicated>, With<FloorMarker>)>
){
    for entity in &floor_query {
        commands.entity(entity).insert((
            RigidBody::Static,
            Collider::cylinder(50.0, 0.1),
            Mesh3d(meshes.add(Cylinder::new(50.0, 0.1))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            GameMask::Floor,
        ));
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,WorldInspectorPlugin::new(),ClientPlugin,AnimationPlugin,CombatantPlugin))
        .add_systems(Startup,default_stuff)
        .add_systems(First,floor_load)
        .run();
}

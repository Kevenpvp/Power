use avian3d::prelude::{Collider, RigidBody};
use bevy::DefaultPlugins;
use bevy::prelude::{default, App, Camera3d, Commands, Dir3, IntoSystemConfigs, PointLight, Startup, Transform, Vec3};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use lightyear::prelude::NetworkTarget;
use lightyear::prelude::server::{Replicate, ReplicationTarget};
use shared::GameMask;
use shared::protocol::{FloorMarker, REPLICATION_GROUP};
use crate::plugins::combatant::CombatantPlugin;
use crate::plugins::connection::{start_server, ServerPlugin};

pub mod plugins;
pub mod systems;


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

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Dir3::Y),
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(50.0, 0.1),
        FloorMarker,
        GameMask::Floor,
        Replicate{
            group: REPLICATION_GROUP,
            target: ReplicationTarget {
                target: NetworkTarget::All,
            },
            ..default()
        }
    ));
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,WorldInspectorPlugin::new(),ServerPlugin, CombatantPlugin))
        .add_systems(Startup,default_stuff.after(start_server))
        .run();
}

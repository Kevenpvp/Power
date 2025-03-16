use bevy::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use avian3d::prelude::*;
use avian3d::sync::{position_to_transform};
use lightyear::prelude::{Key, SharedConfig, TickConfig};
use crate::{InteractNetworkAble, NetworkSide};
use crate::plugins::combatant::CombatantPlugin;
use crate::plugins::statesmachine::StatesMachinePlugin;
use crate::protocol::ProtocolPlugin;

pub const FIXED_TIMESTEP_HZ: f64 = 64.0;
pub const REPLICATION_INTERVAL: Duration = Duration::from_millis(100);
pub const SERVER_PORT: u16 = 2555;
pub const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);
pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), SERVER_PORT);
pub const PROTOCOL_ID: u64 = 1;
pub const PRIVATE_KEY: Key = [5; 32];

pub struct SharedPlugin{
    pub predict_all: bool,
    pub network_side: NetworkSide
}

pub struct Settings{
    pub input_delay_ticks: u16,
    pub correction_ticks_factor: f32,
    pub predict_all: bool
}

impl Plugin for SharedPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(ProtocolPlugin {
            predict_all: self.predict_all,
            network_side: self.network_side.clone()
        });

        app.add_plugins(StatesMachinePlugin);

        app.add_plugins(CombatantPlugin{
            network_side: self.network_side.clone(),
        });

        app.add_plugins(
            PhysicsPlugins::default()
                .build()
                .disable::<PhysicsInterpolationPlugin>(),
        );

        app.add_plugins(PhysicsDebugPlugin::default());

        app.insert_resource(avian3d::sync::SyncConfig {
            transform_to_position: false,
            position_to_transform: true,
            ..default()
        });

        app.insert_resource(SleepingThreshold {
            linear: -0.01,
            angular: -0.01,
        });

        app.add_systems(
            PostUpdate,
            position_to_transform
                .in_set(
                    PhysicsSet::Sync)
                .run_if(|config: Res<avian3d::sync::SyncConfig>| config.position_to_transform),
        );

        app.add_systems(PostUpdate,fix_transform.after(position_to_transform));
    }
}

pub fn shared_configs() -> SharedConfig{
    SharedConfig {
        server_replication_send_interval: REPLICATION_INTERVAL,
        client_replication_send_interval: REPLICATION_INTERVAL,
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
        },
    }
}

pub fn settings() -> Settings{
    Settings{
        input_delay_ticks: 0,
        correction_ticks_factor: 4.0,
        predict_all: false
    }
}

fn fix_transform(
    mut query: Query<(&mut Transform, &Position, &Rotation), (With<RigidBody>, With<InteractNetworkAble>)>
){
    for (mut transform, position, rotation) in query.iter_mut(){
        if transform.translation != position.0 {
            transform.translation = position.0;
        }

        if transform.rotation != rotation.0 {
            transform.rotation = rotation.0;
        }
    }
}

use avian3d::prelude::{AngularVelocity, ComputedMass, ExternalForce, ExternalImpulse, GravityScale, LinearVelocity, Position, Rotation};
use bevy::app::App;
use bevy::prelude::{default, Component, Plugin, Reflect, Transform};
use leafwing_input_manager::{Actionlike, InputControlKind};
use lightyear::prelude::{AppComponentExt, ChannelDirection, InputConfig, LeafwingInputPlugin, ReplicationGroup};
use lightyear::prelude::client::{ComponentSyncMode, LerpFn};
use lightyear::utils::avian3d::{position, rotation};
use lightyear::utils::bevy::TransformLinearInterpolation;
use serde::{Deserialize, Serialize};
use crate::{NetworkSide};
use crate::plugins::combatant::{CombatantMarker, CombatantType};
use crate::plugins::statesmachine::{CurrentStates};

pub struct ProtocolPlugin {
    pub predict_all: bool,
    pub network_side: NetworkSide
}

pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FloorMarker;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect, Serialize, Deserialize)]
pub enum CharacterAction {
    Move,
    Jump
}

impl Actionlike for CharacterAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Move => InputControlKind::DualAxis,
            Self::Jump => InputControlKind::Button
        }
    }
}

impl Plugin for ProtocolPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(LeafwingInputPlugin::<CharacterAction> {
            config: InputConfig::<CharacterAction> {
                rebroadcast_inputs: self.predict_all,
                ..default()
            },
        });

        app.register_component::<FloorMarker>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<CombatantMarker>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<CombatantType>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<GravityScale>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<CurrentStates>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Simple);

        app.register_component::<LinearVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<AngularVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<ExternalForce>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<ExternalImpulse>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<ComputedMass>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<Position>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation_fn(position::lerp)
            .add_interpolation(ComponentSyncMode::Full)
            .add_correction_fn(position::lerp);

        app.register_component::<Rotation>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation_fn(rotation::lerp)
            .add_interpolation(ComponentSyncMode::Full)
            .add_correction_fn(rotation::lerp);

        app.add_interpolation::<Transform>(ComponentSyncMode::None);
        app.add_interpolation_fn::<Transform>(TransformLinearInterpolation::lerp);
    }
}


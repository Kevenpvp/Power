use std::cmp::PartialEq;
use avian3d::prelude::{Collider, Friction, GravityScale, LockedAxes, RigidBody, ShapeHitData};
use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::prelude::{Added, BuildChildren, Bundle, Commands, Component, Entity, EventReader, First, FixedUpdate, GltfAssetLabel, Has, InheritedVisibility, IntoSystemConfigs, KeyCode, Or, Plugin, Query, Res, ResMut, Resource, Transform, With, Without};
use bevy::scene::SceneRoot;
use bevy::utils::default;
use bevy::utils::hashbrown::HashMap;
use leafwing_input_manager::prelude::{ActionState, InputMap, VirtualDPad};
use lightyear::connection::client::ClientConnection;
use lightyear::prelude::{ClientId, Deserialize, NetworkTarget, Serialize};
use lightyear::prelude::client::{Interpolated, NetClient, Predicted};
use lightyear::prelude::server::{ConnectEvent, ControlledBy, Replicate, SyncTarget};
use lightyear::shared::replication::components::Controlled;
use crate::{GameMask, InteractNetworkAble, NetworkSide};
use crate::plugins::statesmachine::CurrentStates;
use crate::protocol::{CharacterAction, REPLICATION_GROUP};
use crate::systems::charactercontroller::{adjust_collider_float, character_walk, check_is_grounded, control_gravity};

#[derive(Resource)]
pub struct CombatantsList(pub HashMap<Entity,Option<ClientId>>);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CombatantMarker;

#[derive(Component)]
pub struct CombatantControlling;

#[derive(Component)]
pub struct CombatantMeshBody;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CombatantType{
    Player,
    Npc
}

#[derive(Component)]
pub struct PlayerCombatant;

#[derive(Component)]
pub struct CharacterController{
    pub shape_hit_data: Option<ShapeHitData>,
    pub grounded: bool
}

#[derive(Bundle)]
pub struct CombatantServerBundle{
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    gravity_scale: GravityScale,
    friction: Friction,
    combatant_type: CombatantType,
    combatant_marker: CombatantMarker,
    network_side: NetworkSide,
    current_states: CurrentStates,
    transform: Transform,
    replicate: Replicate,
    locked_axes: LockedAxes,
    game_mask: GameMask,
    inherited_visibility: InheritedVisibility,
    interact_network_able: InteractNetworkAble,
    action_state: ActionState<CharacterAction>
}

#[derive(Bundle)]
pub struct CombatantClientBundle{
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    friction: Friction,
    network_side: NetworkSide,
    locked_axes: LockedAxes,
    game_mask: GameMask,
    inherited_visibility: InheritedVisibility,
    interact_network_able: InteractNetworkAble
}

#[derive(Bundle)]
pub struct CombatantMeshBundle{
    combatant_mesh_body: CombatantMeshBody,
    transform: Transform,
    scene_root: SceneRoot
}

pub struct CombatantPlugin{
    pub network_side: NetworkSide,
}

impl Default for CharacterController{
    fn default()->Self{
        Self{
            shape_hit_data: None,
            grounded: true
        }
    }
}

impl Default for CombatantMeshBundle{
    fn default()->Self{
        Self{
            combatant_mesh_body: CombatantMeshBody,
            transform: Transform::from_xyz(0.0,-0.9,0.0),
            scene_root: Default::default()
        }
    }
}

impl Default for CombatantServerBundle{
    fn default() -> CombatantServerBundle{
        CombatantServerBundle{
            character_controller: CharacterController::default(),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::capsule(0.3,1.0),
            gravity_scale: GravityScale(0.0),
            friction: Friction::new(1.0),
            combatant_type: CombatantType::Player,
            combatant_marker: CombatantMarker,
            network_side: NetworkSide::Server,
            current_states: CurrentStates::default(),
            transform: Transform::from_xyz(0.0, 0.85, 0.0),
            replicate: Replicate::default(),
            locked_axes: LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            game_mask: GameMask::Combatant,
            inherited_visibility: InheritedVisibility::VISIBLE,
            interact_network_able: InteractNetworkAble,
            action_state: ActionState::<CharacterAction>::default()
        }
    }
}

impl Default for CombatantClientBundle{
    fn default() -> CombatantClientBundle{
        CombatantClientBundle{
            character_controller: CharacterController::default(),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::capsule(0.3,1.0),
            friction: Friction::new(1.0),
            network_side: NetworkSide::Client,
            locked_axes: LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            game_mask: GameMask::Combatant,
            inherited_visibility: InheritedVisibility::VISIBLE,
            interact_network_able: InteractNetworkAble
        }
    }
}

impl Plugin for CombatantPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CombatantsList(HashMap::new()));
        if self.network_side == NetworkSide::Client {
            app.add_systems(First,client_combatant_added);
        }else {
            app.add_systems(First,create_player_combatant);
        }

        app.add_systems(FixedUpdate,(check_is_grounded,adjust_collider_float,character_walk,control_gravity).chain());
    }
}

fn client_combatant_added(
    mut commands: Commands,
    combatant_query: Query<(Entity,Has<Controlled>), (Or<(Added<Predicted>, Added<Interpolated>)>, With<CombatantMarker>, Without<NetworkSide>)>,
    connection: Res<ClientConnection>,
    asset_server: Res<AssetServer>,
    mut combatants_list: ResMut<CombatantsList>,
){
    for (entity,is_controlled) in combatant_query.iter(){
        let mesh_entity = commands.spawn(
            CombatantMeshBundle{
                scene_root: SceneRoot(
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Default.glb")),
                ),
                ..default()
            }
        ).id();
        let entity = commands.entity(entity).insert(CombatantClientBundle::default())
            .add_child(mesh_entity)
            .id();

        if is_controlled {
            commands.entity(entity).insert((
                PlayerCombatant,
                InputMap::new([(CharacterAction::Jump,KeyCode::Space)]).with_dual_axis(CharacterAction::Move, VirtualDPad::wasd()),
            ));
        }

        combatants_list.0.insert(entity,Some(connection.id()));
    }
}

pub fn create_player_combatant(
    mut connections: EventReader<ConnectEvent>,
    mut commands: Commands,
    mut combatants_list: ResMut<CombatantsList>
){
    for connection in connections.read() {
        let client_id = connection.client_id;

        let entity = commands.spawn(CombatantServerBundle{
            replicate: Replicate {
                controlled_by: ControlledBy {
                    target: NetworkTarget::Single(client_id),
                    ..default()
                },
                group: REPLICATION_GROUP,
                sync: SyncTarget {
                    prediction: NetworkTarget::Single(client_id),
                    interpolation: NetworkTarget::AllExceptSingle(client_id),
                    ..default()
                },
                ..default()
            },
            ..default()
        });

        combatants_list.0.insert(entity.id(),Some(client_id));
    }
}
use avian3d::prelude::PhysicsLayer;
use bevy::prelude::Component;

pub mod protocol;
pub mod plugins;
pub mod systems;

#[derive(Component,PhysicsLayer,Clone,Copy,Debug,Default)]
pub enum GameMask{
    #[default]
    Default,
    Floor,
    Combatant
}
#[derive(Component)]
pub struct InteractNetworkAble;

#[derive(Component, PartialEq, Clone, Copy)]
pub enum NetworkSide{
    Client,
    Server
}
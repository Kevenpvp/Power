use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, Startup};
use bevy::utils::default;
use lightyear::connection::server::{IoConfig, NetConfig};
use lightyear::prelude::server::{NetcodeConfig, ServerCommandsExt, ServerConfig, ServerPlugins, ServerTransport};
use shared::NetworkSide;
use shared::plugins::shared::{settings, shared_configs, SharedPlugin, PRIVATE_KEY, PROTOCOL_ID, SERVER_ADDR};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let settings = settings();

        app.add_plugins((setup_server_plugins(),SharedPlugin{
            predict_all: settings.predict_all,
            network_side: NetworkSide::Server
        }));
        app.add_systems(Startup,start_server);
    }
}

fn setup_net_config() -> NetConfig{
    let io_config = IoConfig{
        transport: ServerTransport::UdpSocket(SERVER_ADDR),
        ..default()
    };

    let netcode_config = NetcodeConfig::default()
        .with_protocol_id(PROTOCOL_ID)
        .with_key(PRIVATE_KEY);


    NetConfig::Netcode{
        config: netcode_config,
        io: io_config,
    }
}

fn setup_server_plugins() -> ServerPlugins {
    ServerPlugins::new(ServerConfig{
        shared: shared_configs(),
        net: vec![setup_net_config()],
        ..default()
    })
}

pub fn start_server(mut commands: Commands) {
    commands.start_server();
}
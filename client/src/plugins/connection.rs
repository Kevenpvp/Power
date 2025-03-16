use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, Startup};
use bevy::utils::default;
use lightyear::connection::client::{IoConfig, NetConfig};
use lightyear::prelude::client::{Authentication, ClientCommandsExt, ClientConfig, ClientPlugins, ClientTransport, PredictionConfig};
use shared::NetworkSide;
use shared::plugins::shared::{settings, shared_configs, Settings, SharedPlugin, CLIENT_ADDR, PRIVATE_KEY, PROTOCOL_ID, SERVER_ADDR};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        let settings = settings();

        app.add_plugins((setup_client_plugins(&settings), SharedPlugin{
            predict_all: settings.predict_all,
            network_side: NetworkSide::Client
        }));
        app.add_systems(Startup,connect_to_server);
    }
}

fn setup_net_config() -> NetConfig{
    let io_config = IoConfig{
        transport: ClientTransport::UdpSocket(CLIENT_ADDR),
        ..default()
    };

    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: 1,
        private_key: PRIVATE_KEY,
        protocol_id: PROTOCOL_ID,
    };

    NetConfig::Netcode {
        auth,
        config: Default::default(),
        io: io_config
    }
}

fn setup_client_plugins(settings: &Settings) -> ClientPlugins {
    let mut prediction_config = PredictionConfig::default();

    prediction_config.set_fixed_input_delay_ticks(settings.input_delay_ticks);
    prediction_config.correction_ticks_factor = settings.correction_ticks_factor;

    ClientPlugins::new(ClientConfig {
        shared: shared_configs(),
        net: setup_net_config(),
        prediction: prediction_config,
        ..default()
    })
}

fn connect_to_server(mut commands: Commands) {
    commands.connect_client();
}
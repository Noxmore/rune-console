use bevy::prelude::*;
use bevy_console::{ConsoleConfiguration, ConsolePlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ConsolePlugin::default()))
        .insert_resource(ConsoleConfiguration {
            toggle_keys: vec![
                KeyCode::Backquote,
                KeyCode::F1,
            ],
            ..Default::default()
        })
        .run();
}

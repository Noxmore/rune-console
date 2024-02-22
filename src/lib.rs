#![doc = include_str ! ("../README.md")]

use bevy::prelude::*;
pub use bevy_console_derive::ConsoleCommand;
use bevy_egui::EguiPlugin;
pub use clap;

use crate::commands::clear::{clear_command, ClearCommand};
use crate::commands::exit::{exit_command, ExitCommand};
use crate::commands::help::{help_command, HelpCommand};
/* pub use crate::console::{
    AddConsoleCommand, Command, ConsoleCommand, ConsoleCommandEntered, ConsoleConfiguration,
    ConsoleOpen, NamedCommand, PrintConsoleLine, ToggleConsoleKey,
	ConsoleLine, ConsoleLineComponent,
}; */
pub use crate::console::*;
// pub use color::{Style, StyledStr};

use crate::console::{console_ui, receive_console_line, ConsoleState};

// mod color;
mod commands;
mod console;
mod macros;

/// Console plugin.
#[derive(Default)]
pub struct ConsolePlugin {
	/// Whether or not to add the egui plugin and render the console ui, useful to turn off for servers.
	pub egui: bool,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
/// The SystemSet for console/command related systems
pub enum ConsoleSet {
    /// Systems operating the console UI (the input layer)
    ConsoleUI,

    /// Systems executing console commands (the functionality layer).
    /// All command handler systems are added to this set
    Commands,

    /// Systems running after command systems, which depend on the fact commands have executed beforehand (the output layer).
    /// For example a system which makes use of [`PrintConsoleLine`] events should be placed in this set to be able to receive
    /// New lines to print in the same frame
    PostCommands,
}

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConsoleConfiguration>()
            .init_resource::<ConsoleState>()
            .init_resource::<ConsoleOpen>()
            .add_event::<ConsoleCommandEntered>()
            .add_event::<PrintConsoleLine>()
            .add_console_command::<ClearCommand, _>(clear_command)
            .add_console_command::<ExitCommand, _>(exit_command)
            .add_console_command::<HelpCommand, _>(help_command)
            .add_systems(
                Update,
                (
                    receive_console_line.in_set(ConsoleSet::PostCommands),
                ),
            )
            .configure_sets(
                Update,
                (
                    ConsoleSet::Commands
                        .after(ConsoleSet::ConsoleUI),
                    ConsoleSet::PostCommands.after(ConsoleSet::Commands),
                ),
            );

		if self.egui {
			app.add_systems(Update, console_ui.in_set(ConsoleSet::ConsoleUI));
		}

        // Don't initialize an egui plugin if one already exists.
        // This can happen if another plugin is using egui and was installed before us.
        if self.egui && !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
    }
}

use clap::Parser;

mod config;
mod ui;

use crate::ui::UiPlugin;

/// Radio telescope controller program.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Run in daemon-mode (no UI, for microcontrollers).
    #[arg(short, long, default_value_t = false)]
    daemon: bool,
}

fn main() {
    let args = Args::parse();

    if args.daemon {
        println!("Running in daemon (server) mode...");
        let config = config::parse_server_config();
    } else {
        println!("Running in client mode...");
        bevy::prelude::App::new()
            .init_resource::<config::ClientConfig>()
            .add_plugins(UiPlugin)
            .run();
    }
}

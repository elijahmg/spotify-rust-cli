mod args;
mod handlers;
mod oauth;
mod spotify;
use crate::spotify::{get_device, get_spotify};

use clap::Command;

// use crate::args::Command;

// TODO: set client device into file

#[tokio::main]
async fn main() {
  env_logger::init();

  let matches = Command::new("Spotify cli")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .subcommand(args::playback_subcommand())
    .get_matches();

  let spotify = get_spotify().await;
  let device_id = get_device(&spotify).await.unwrap();

  if let Some(cmd) = matches.subcommand_name() {
    let sub_matches = matches.subcommand_matches(cmd).unwrap();

    match cmd {
      "playback" => {
        if sub_matches.get_flag("resume") {
          handlers::resume_playback(&spotify, &device_id).await;
        }

        if sub_matches.get_flag("pause") {
          handlers::pause_playback(&spotify, &device_id).await;
        }
      }
      _ => unimplemented!(),
    }
  }
}

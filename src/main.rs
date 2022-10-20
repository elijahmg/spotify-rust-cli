mod args;
mod device;
mod handlers;
mod oauth;
mod spotify;

use crate::spotify::get_spotify;

use crate::device::DeviceConfig;
use crate::handlers::{playback, track};
use clap::Command;

// TODO: set client device into file

#[tokio::main]
async fn main() {
  env_logger::init();

  let matches = Command::new("Spotify cli")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .subcommand(args::playback_subcommand())
    .subcommand(args::track_subcommand())
    .get_matches();

  let spotify = get_spotify().await;

  let mut device_config = DeviceConfig::new();

  // load device id
  device_config.load(&spotify).await.unwrap();

  let device_id = device_config
    .get_device_id()
    .expect("Failed to get device id");

  if let Some(cmd) = matches.subcommand_name() {
    let sub_matches = matches.subcommand_matches(cmd).unwrap();

    match cmd {
      "playback" => {
        if sub_matches.get_flag("resume") {
          playback::resume_playback(&spotify, device_id).await;
        }

        if sub_matches.get_flag("pause") {
          playback::pause_playback(&spotify, device_id).await;
        }
      }
      "track" => {
        if sub_matches.get_flag("next") {
          track::next_track(&spotify, device_id).await;
        }

        if sub_matches.get_flag("prev") {
          track::prev_track(&spotify, device_id).await;
        }

        if sub_matches.get_flag("like") {
          track::like_track(&spotify).await;
        }

        if sub_matches.get_flag("unlike") {
          track::unlike_track(&spotify).await;
        }
      }
      _ => unimplemented!(),
    }
  }
}

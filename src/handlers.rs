use rspotify::{prelude::*, AuthCodeSpotify};

pub async fn resume_playback(spotify: &AuthCodeSpotify, device_id: &str) {
  match spotify.resume_playback(Some(device_id), None).await {
    Ok(_) => {}
    Err(_) => {
      println!("Something went wrong with resume_playback");
    }
  };
}

pub async fn pause_playback(spotify: &AuthCodeSpotify, device_id: &str) {
  match spotify.pause_playback(Some(device_id)).await {
    Ok(_) => {}
    Err(_) => {
      println!("Something went wrong with pause_playback");
    }
  };
}

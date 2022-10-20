use rspotify::model::{
  AdditionalType, Country, CurrentlyPlayingContext, Market, PlayableItem,
};
use rspotify::{prelude::*, AuthCodeSpotify};

pub async fn next_track(spotify: &AuthCodeSpotify, device_id: &str) {
  match spotify.next_track(Some(device_id)).await {
    Ok(_) => {}
    Err(_) => {
      println!("Something went wrong with next_track")
    }
  }
}

pub async fn prev_track(spotify: &AuthCodeSpotify, device_id: &str) {
  match spotify.previous_track(Some(device_id)).await {
    Ok(_) => {}
    Err(_) => {
      println!("Something went wrong with next_track")
    }
  }
}

pub async fn like_track(spotify: &AuthCodeSpotify) {
  let currently_playing_context = get_playback_state(spotify).await.unwrap();
  match currently_playing_context.item.unwrap() {
    PlayableItem::Track(track) => {
      let track_id = track.id.unwrap();

      match spotify.current_user_saved_tracks_add([&track_id]).await {
        Ok(_) => {}
        Err(_) => println!("Failed to like the song"),
      }
    }
    PlayableItem::Episode(_) => {
      unimplemented!()
    }
  }
}

pub async fn unlike_track(spotify: &AuthCodeSpotify) {
  let currently_playing_context = get_playback_state(spotify).await.unwrap();
  match currently_playing_context.item.unwrap() {
    PlayableItem::Track(track) => {
      let track_id = track.id.unwrap();

      match spotify.current_user_saved_tracks_delete([&track_id]).await {
        Ok(_) => {}
        Err(_) => println!("Failed to like the song"),
      }
    }
    PlayableItem::Episode(_) => {
      unimplemented!()
    }
  }
}

pub async fn get_playback_state(
  spotify: &AuthCodeSpotify,
) -> Option<CurrentlyPlayingContext> {
  let country = Market::Country(Country::CzechRepublic);
  let additional_types = [AdditionalType::Track, AdditionalType::Episode];

  match spotify
    .current_playing(Some(&country), Some(&additional_types))
    .await
    .unwrap()
  {
    Some(currently_playing_context) => Some(currently_playing_context),
    _ => {
      println!("Failed to get current context");
      None
    }
  }
}

use rspotify::{model::PlayableItem, prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};
use anyhow::{Result as Result, anyhow};

pub fn get_authed_spotify() -> AuthCodeSpotify {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!(
            "user-read-playback-state", 
            "user-modify-playback-state", 
            "user-read-currently-playing"
        )).unwrap();

    let spotify = AuthCodeSpotify::new(creds.clone(), oauth.clone());

    // Obtaining the access token
    let url = spotify.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
    spotify.prompt_for_token(&url).unwrap();

    spotify
}

/// tries to return the current playing item (as rspotify::model::PlayableItem) using RSpotify and returns None if either the CurrentlyPlayingContext or its PlayableItem field come back None and reuturns an forwards an Error if the spotify API has an error.
fn try_get_current_playing_item(spotify: &AuthCodeSpotify) -> Result<Option<PlayableItem>> {
    let Some(current_user_playing_item) = spotify.current_user_playing_item()? else {
        return Ok(None);
    };
    return match current_user_playing_item.item {
        Some(playable_item) => Ok(Some(playable_item)),
        None => Ok(None)
    }
}

/// tries to get the name field of the current_user_playing_item (see its function documentation
/// for details on return possibilities). Is None if the try_get_current_playing_item returns
/// Ok(None). Returns Err(...) if it returns an Error. Otherwise, it returns the field, as
/// expected.
pub fn try_get_current_playing_item_name(spotify: &AuthCodeSpotify) -> Result<Option<String>> {
    let Some(current_user_playing_item) = try_get_current_playing_item(spotify)? else {
        return Ok(None);
    };
    match current_user_playing_item {
        PlayableItem::Track(full_track) => Ok(Some(full_track.name)),
        PlayableItem::Episode(full_episode) => Ok(Some(full_episode.name)),
    }
}

// TODO: create api access to item album/track image

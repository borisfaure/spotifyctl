use clap::Command;
use log::debug;
use rspotify::model::PlayableItem;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, ClientResult, Config, Credentials, OAuth};

/// Get a string of the current playing song if any
async fn get_playing_string(
    spotify: AuthCodeSpotify,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let playing = spotify.current_user_playing_item().await?;
    if let Some(p) = playing {
        if let Some(pi) = p.item {
            match pi {
                PlayableItem::Track(f) => {
                    if !f.artists.is_empty() {
                        debug!("{} - {}", f.artists[0].name, f.name);
                        Ok(Some(format!("{} - {}", f.artists[0].name, f.name)))
                    } else {
                        debug!("{}", f.name);
                        Ok(Some(f.name.to_string()))
                    }
                }
                PlayableItem::Episode(e) => {
                    debug!("{} - {}", e.show.name, e.name);
                    Ok(Some(format!("{} - {}", e.show.name, e.name)))
                }
            }
        } else {
            debug!("no item");
            Ok(None)
        }
    } else {
        debug!("not playing");
        Ok(None)
    }
}

/// Play the next song/episode
async fn next(spotify: AuthCodeSpotify) -> ClientResult<()> {
    spotify.next_track(None).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    lovely_env_logger::init(lovely_env_logger::Config::new_reltime());
    let matches = Command::new("Spotify Control")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Boris Faure <boris.faure@gmail.com>")
        .about("My own dumb spotify controller")
        .subcommand(Command::new("get").about("get the currently playing song/episode"))
        .subcommand(Command::new("next").about("Play the next song/episode"))
        .get_matches();

    let config_dir_opt = dirs::config_dir();
    if config_dir_opt.is_none() {
        panic!("unable to find configuration directory");
    }
    let mut cache_path = config_dir_opt.unwrap();
    cache_path.push(".spotify.token");
    // Enabling automatic token refreshing in the config
    let config = Config {
        token_refreshing: true,
        token_cached: true,
        cache_path,
        ..Default::default()
    };

    // Using every possible scope
    let scopes = scopes!("user-read-playback-state", "user-modify-playback-state");
    let oauth_opt = OAuth::from_env(scopes);
    if oauth_opt.is_none() {
        panic!("unable to create oauth from environment variables");
    }
    let oauth = oauth_opt.unwrap();

    let creds = Credentials::from_env().unwrap();
    let spotify = AuthCodeSpotify::with_config(creds.clone(), oauth, config.clone());
    let url = spotify.get_authorize_url(false).unwrap();

    // This function requires the `cli` feature enabled.
    spotify
        .prompt_for_token(&url)
        .await
        .expect("couldn't authenticate successfully");

    if matches.subcommand_matches("get").is_some() {
        if let Some(s) = get_playing_string(spotify).await? {
            println!("{}", s);
        }
    } else if matches.subcommand_matches("next").is_some() {
        next(spotify).await?
    }
    Ok(())
}

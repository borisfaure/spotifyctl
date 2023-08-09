use clap::{App, Command};
use dirs;
use rspotify::model::PlayableItem;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Config, Credentials, OAuth};

/// Print the current playing song if any
async fn get_playing(spotify: AuthCodeSpotify) -> Result<(), Box<dyn std::error::Error>> {
    let playing = spotify.current_user_playing_item().await?;
    if let Some(p) = playing {
        if let Some(pi) = p.item {
            match pi {
                PlayableItem::Track(f) => {
                    if f.artists.len() > 0 {
                        println!("{} - {}", f.artists[0].name, f.name);
                    } else {
                        println!("{}", f.name);
                    }
                }
                PlayableItem::Episode(e) => {
                    println!("{} - {}", e.show.name, e.name);
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    lovely_env_logger::init(lovely_env_logger::Config::new_reltime());
    let _matches = App::new("Spotify Control")
        .version("0.1")
        .author("Boris Faure <boris.faure@gmail.com>")
        .about("My own dumb spotify controller")
        .subcommand(Command::new("get").about("get the currently playing song/episode"))
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

    get_playing(spotify).await
}

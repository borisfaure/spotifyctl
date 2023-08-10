#![warn(missing_docs)]

//!  A simple tool to control spotify from the command line.
//!  Based on [rspotify](https://github.com/ramsayleung/rspotify).
//!
//!  SpotifyCtl has few commmands:
//!
//! - `get`: Display the currently playing track or episode.
//! - `previous`: Restart the current track or skip to the previous track.
//! - `next`: Skip to the next track.
//! - `play-pause`: Pause or resume playback.

use chrono::Duration;
use clap::{Arg, Command};
use log::debug;
use rspotify::model::PlayableItem;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, ClientResult, Config, Credentials, OAuth};
use std::io::{self, Write};

/// Build a Command
fn build_cli() -> Command {
    Command::new("Spotify Control")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Boris Faure <boris.faure@gmail.com>")
        .about("My own dumb spotify controller")
        .subcommand(Command::new("get").about("Get the currently playing song/episode"))
        .subcommand(
            Command::new("previous").about("Restart the current track or skip to the previous one")
                .arg(
                    Arg::new("max-progress")
                        .long("max-progress")
                        .short('m')
                        .value_name("DURATION")
                        .num_args(1)
                        .value_parser(clap::value_parser!(i64))
                        .default_value("15")
                        .help("Skip to the previous track if progress is lower than this duration (in seconds)"),
                ),
        )
        .subcommand(Command::new("next").about("Skip to the next track/episode"))
        .subcommand(Command::new("play-pause").about("Pause or resume playback"))
}

/// Get a string of the current playing song if any
///
/// * `spotify` - The Spotify API helper
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
        debug!("no playing result");
        Ok(None)
    }
}

/// Skip to the next song/episode
///
/// * `spotify` - The Spotify API helper
async fn next(spotify: AuthCodeSpotify) -> ClientResult<()> {
    spotify.next_track(None).await
}

/// Restart the song/episode or skip to the previous song
///
/// * `spotify` - The Spotify API helper
/// * `max_progress` - If current track progress is lower than this, then skip
///                    to the previous track. Otherwise, play the track from
///                    the beginning
async fn previous(spotify: AuthCodeSpotify, max_progress: i64) -> ClientResult<()> {
    debug!("max progress is {}", max_progress);
    let playing = spotify.current_user_playing_item().await?;
    if let Some(p) = playing {
        if p.is_playing {
            if let Some(d) = p.progress {
                let progress = d.num_seconds();
                debug!(
                    "progress is {} while max_delay is {}",
                    progress, max_progress
                );
                if progress < max_progress {
                    debug!("skip to previous");
                    return spotify.previous_track(None).await;
                }
            }
            debug!("seek to 0");
            return spotify.seek_track(Duration::seconds(0), None).await;
        } else {
            debug!("not playing");
        }
    } else {
        debug!("no playing result");
    }
    Ok(())
}

/// Pause or resume playback
///
/// * `spotify` - The Spotify API helper
async fn play_pause(spotify: AuthCodeSpotify) -> ClientResult<()> {
    let playing = spotify.current_user_playing_item().await?;
    if let Some(p) = playing {
        if p.is_playing {
            debug!("is playing");
            spotify.pause_playback(None).await
        } else {
            debug!("is not playing");
            spotify.resume_playback(None, None).await
        }
    } else {
        debug!("no playing result");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    lovely_env_logger::init(lovely_env_logger::Config::new_reltime());

    let matches = build_cli().get_matches();

    let config_dir_opt = dirs::config_dir();
    if config_dir_opt.is_none() {
        panic!("unable to find configuration directory");
    }
    let mut cache_path = config_dir_opt.unwrap();
    cache_path.push(".spotifyctl.token");
    // Enabling automatic token refreshing in the config
    let config = Config {
        token_refreshing: true,
        token_cached: true,
        cache_path,
        ..Default::default()
    };

    // Using minimal scopes to work with all the commands
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
    } else if let Some(m) = matches.subcommand_matches("previous") {
        previous(spotify, *m.get_one::<i64>("max-progress").unwrap()).await?
    } else if matches.subcommand_matches("play-pause").is_some() {
        play_pause(spotify).await?
    } else {
        let mut out = io::stdout();
        out.write_all(b"Invalid or missing subcommand\n\n")?;
        let help = build_cli().render_help();
        write!(out, "{}", help)?;
    }
    Ok(())
}

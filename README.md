# SpotifyCtl

A simple tool to control spotify from the command line.
Based on [rspotify](https://github.com/ramsayleung/rspotify).

## Features

SpotifyCtl has few commmands:

- `get`: Display the currently playing song or episode.
- `previous`: Restart the current track or get to the previous track.
- `next`: Get to the next track.

## Installation

SpotifyCtl is written in Rust.
First, one need to [Install a rust toolchain](https://www.rust-lang.org/tools/install).
And then run:

    cargo install --path .

## Configuration

1. One need to create a spotify app on [the dedicated page](https://developer.spotify.com/dashboard/applications).
2. You will get a `Client Id` and a `Client Secret`. Note them down.
3. Edit the settings of your app to add a `Redirect URI`. I suggest using
   `https://localhost:8888/callback`. This URI do not need to be accessible.
4. Add any user email that will be able to use `spotifyctl`.
5. Export the following environment variables with the values noted down:

```sh
export RSPOTIFY_CLIENT_ID=ef0fbc0adc633de52214e7a211a13310
export RSPOTIFY_CLIENT_SECRET=08c77cdf9a8df31ffac2cd03eff0a748
export RSPOTIFY_REDIRECT_URI=https://localhost:8888/callback
```

6. Run `spotifyctl get` in a terminal. This should start a browser when you
   will to let your spotify user accept to connect to your new spotify app.
7. You will redirected to an invalid webpage. Note that URL and past it to the
   terminal where you ran `spotifyctl get`.
8. This will occur only once since `spotifyctl` will store a renewable token
   on the disk and you will not have to authenticate using a browser again.
9. Run any other `spotifyctl` command whenever you want, but always with the
   previously defined environment variables.

# Spotify CLI

### This project has been made for fun and inspired by [Spotify tui](https://github.com/Rigellute/spotify-tui)

## Connecting to Spotify’s API

`spotify-cli` needs to connect to Spotify’s API in order to find music by
name, play tracks etc.

Instructions on how to set this up will be shown when you first run the app.

But here they are again:

1. Go to the [Spotify dashboard](https://developer.spotify.com/dashboard/applications)
1. Click `Create an app`
    - You now can see your `Client ID` and `Client Secret`
1. Now click `Edit Settings`
1. Add `http://localhost:8888/callback` to the Redirect URIs
1. Scroll down and click `Save`
1. You are now ready to authenticate with Spotify!
1. Go back to the terminal
1. [to improve] Go to terminal settings (~/.zshrc for zsh) and add export like:
```dotenv
export RSPOTIFY_CLIENT_ID="YOUR_CLIENT_ID"
export RSPOTIFY_CLIENT_SECRET="YOUR_SECRET_ID"
export RSPOTIFY_REDIRECT_URI="http://localhost:8888/callback"
```

## Usage [to improve]

First you will need to build it by `cargo run --release`
Then put spf bin into desired folder and set it as executable.
For zsh it's like `export PATH="$HOME/.spf/bin:$PATH"` into `~/.zshrc`
After in new terminal session


Run in terminal `spf pb -r/-p`.
Currently, this supports only pause and resume playback


## More is coming 🚅
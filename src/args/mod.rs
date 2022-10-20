use clap::{arg, ArgAction, Command};

pub fn playback_subcommand() -> Command {
  Command::new("playback")
    .visible_alias("pb")
    .about("Change playback state")
    .arg(arg!(-p --pause "Pause playback").action(ArgAction::SetTrue))
    .arg(arg!(-r --resume "Resume playback").action(ArgAction::SetTrue))
}

pub fn track_subcommand() -> Command {
  Command::new("track")
    .visible_alias("tr")
    .about("Action with current track")
    .arg(arg!(-n --next "Next track").action(ArgAction::SetTrue))
    .arg(arg!(-p --prev "Prev track").action(ArgAction::SetTrue))
    .arg(arg!(-l --like "Like track").action(ArgAction::SetTrue))
    .arg(arg!(-u --unlike "Un-like track").action(ArgAction::SetTrue))
}

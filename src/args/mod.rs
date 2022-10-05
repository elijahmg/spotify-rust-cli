use clap::{arg, ArgAction, Command};

pub fn playback_subcommand() -> Command {
  Command::new("playback")
    .visible_alias("pb")
    .about("Change playback state")
    .arg(arg!(-p --pause "Pause playback").action(ArgAction::SetTrue))
    .arg(arg!(-r --resume "Resume playback").action(ArgAction::SetTrue))
}

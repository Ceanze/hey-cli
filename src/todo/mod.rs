use clap::{Args, Subcommand};

#[derive(Args)]
pub struct Command {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	Add(add::Command),
	Show(show::Command),
	Check(check::Command)
}

pub fn execute(command: Command) -> anyhow::Result<()> {
	match command.command {
		Commands::Add(input) => add::execute(input),
		Commands::Show(input) => show::execute(input),
		Commands::Check(input) => check::execute(input),
	}
}

mod add;
mod show;
mod check;
mod data;
mod utils;
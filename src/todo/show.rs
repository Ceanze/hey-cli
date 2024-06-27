use clap::Args;

use super::utils;

#[derive(Args)]
pub struct Command {}

pub fn execute(_command: Command) -> anyhow::Result<()> {
	let todos = utils::get_todos()?;

	for todo in todos {
		println!("{}", utils::format_todo(&todo, true));
	}

	Ok(())
}
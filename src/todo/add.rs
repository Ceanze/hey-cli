use clap::Args;

use colored::Colorize;

use super::{data::Todo, utils};

#[derive(Args)]
pub struct Command {
	task: Vec<String>
}

pub fn execute(command: Command) -> anyhow::Result<()> {
	let task = command.task.join(" ");
	add(&task)?;

	println!("📝 {}", "Todo added!".green());

	Ok(())
}

fn add(task: &str) -> anyhow::Result<()> {
	let mut todos = utils::get_todos()?;
	todos.push(Todo{timestamp: chrono::Utc::now(), task: task.to_string()});
	utils::write_todos(todos)?;

	Ok(())
}
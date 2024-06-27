use clap::Args;
use std::io::Write;

use super::{data::Todo, utils};

#[derive(Args)]
pub struct Command {}

pub fn execute(_command: Command) -> anyhow::Result<()> {
	let todos = utils::get_todos()?;
	if todos.is_empty() {
		println!("📭 Nothing to check");
		return Ok(());
	}

	if let Some(selection) = get_selection(&todos) {
		remove(&selection, todos)?;
	}

	Ok(())
}

fn get_selection(todos: &Vec<Todo>) -> Option<String> {
	let mut fzf = std::process::Command::new("fzf")
		.stdin(std::process::Stdio::piped())
		.stdout(std::process::Stdio::piped())
		.spawn()
		.expect("Failed to start fzf");

	{
		let stdin = fzf.stdin.as_mut().expect("Failed to open stdin");
		for todo in todos {
			writeln!(stdin, "{}", utils::format_todo(&todo, false)).expect("Failed to write to stdin");
		}
	}

	let output = fzf
		.wait_with_output()
		.expect("Failed to read fzf output");

	if output.status.success() {
		let selection = String::from_utf8_lossy(&output.stdout);
		Some(selection.trim().to_string())
	} else {
		None
	}
}

fn remove(selection: &str, mut todos: Vec<Todo>) -> anyhow::Result<()> {
	// TODO: Check that the correct item is being removed, an index has to be used instead of a raw string
	if let Some(index) = todos.iter().position(|todo| utils::format_todo(todo, false) == selection) {
		let todo = todos.remove(index);
		utils::write_todos(todos)?;

		println!("✅ Marked '{}' as done", todo.task);
		Ok(())
	} else {
		Err(anyhow::format_err!("Nothing selected"))
	}
}
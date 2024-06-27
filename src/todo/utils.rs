use colored::Colorize;

use super::data::{self, TodoStorage, Todo};
use crate::paths::{self};

pub fn get_todos() -> anyhow::Result<Vec<data::Todo>> {
	create_file_if_needed()?;

	let content = std::fs::read_to_string(paths::get_todo_path())?;
	let data: TodoStorage = serde_yaml::from_str(&content)?;

	Ok(data.todos)
}

pub fn write_todos(todos: Vec<Todo>) -> anyhow::Result<()> {
	create_file_if_needed()?;

	let content = serde_yaml::to_string(&TodoStorage{todos: todos})?;
	std::fs::write(paths::get_todo_path(), content)?;

	Ok(())
}

pub fn format_todo(todo: &Todo, colors: bool) -> String {
	if colors {
		format!("{} {}", todo.timestamp.with_timezone(&chrono::Local).format("%d/%m/%y %H:%M").to_string().cyan(), todo.task)
	} else {
		format!("{} {}", todo.timestamp.with_timezone(&chrono::Local).format("%d/%m/%y %H:%M"), todo.task)
	}
}

fn create_file_if_needed() -> anyhow::Result<()> {
	let path = paths::get_todo_path();
	if !path.exists() {
		if let Some(parent) = path.parent() {
			std::fs::create_dir_all(parent)?;
		}

		// Create the file
		std::fs::File::create(&path)?;

		// Write initial data
		let todos = Vec::new();
		let content = serde_yaml::to_string(&TodoStorage{todos: todos})?;
		std::fs::write(paths::get_todo_path(), content)?;
	}

	Ok(())
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct TodoStorage {
	pub todos: Vec<Todo>
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Todo {
	pub timestamp: chrono::DateTime<chrono::Utc>,
	pub task: String
}
pub fn get_config_path() -> std::path::PathBuf {
	get_home_dir().join(".hey/config.yaml")
}

pub fn get_todo_path() -> std::path::PathBuf {
	get_home_dir().join(".hey/todo.yaml")
}

fn get_home_dir() -> std::path::PathBuf {
	dirs::home_dir().expect("A home directory could not be found to place the config. Cannot recover, please report this error if happens.")
}
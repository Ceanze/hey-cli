mod remind;
mod todo;
mod paths;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "Hey!")]
#[command(about = "Hey is a to quickly write down your thoughts", long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,

	#[arg(last = true)]
	free_text: Vec<String>
}

/*
Commands examples:

All commands should be available as synonyms as well by calling "hey -- "

1. Remind: hey remind me to do X at Y               - Adds a reminder "do X" that gets notified at Y
2. Note: hey note down that it is Tuesday           - Adds a generic note "it is Tueday"
3. Add: hey add to list tool ideas write a new cli  - Adds to already existing list "tool ideas" "write a new cli"
4. Create: hey create a list called tool ideas      - Creates a new list called "tool ideas". This makes it possible to have spaces in list names when parsing
5a. Show: hey show me todo                          - Shows all the items in the list "todo"
5b. Show: hey show me all lists                     - Shows all the available lists
5c. Show: hey show me all lists where X is          - (Non-prio) Shows all the lists where item/string is existing, i.e. a filter
*/

#[derive(Subcommand)]
enum Commands {
	Remind(remind::Command),
	Todo(todo::Command)
}

fn main() {
	let res = run();

	if let Some(err) = res.err() {
		println!("🚩 {}", "An error occured!".red());
		println!("Error: {}", err);
	}
}

fn run() -> anyhow::Result<()> {
	let cli = Cli::parse();

	if let Some(commands) = cli.command {
		match commands {
			Commands::Remind(input) => remind::execute(input),
			Commands::Todo(input) => todo::execute(input)
		}
	} else if !cli.free_text.is_empty() {
		println!("Not implemented yet. Text inputed: {}", cli.free_text.join(" "));
		Ok(())
	} else {
		Ok(())
	}
}

// hey todo add
// hey todo show
// hey todo check
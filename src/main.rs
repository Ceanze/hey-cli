mod remind;
mod note;
mod add;
mod create;
mod show;
mod thesaurus;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Hey!")]
#[command(about = "Hey is a to quickly write down your thoughts", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
    Remind { input: Vec<String> },
    Note { input: Vec<String> },
    Add { input: Vec<String> },
    Create { input: Vec<String> },
    Show { input: Vec<String> }
}

fn main() {
    let cli = Cli::parse();

    let mut thesaurus = thesaurus::Thesaurus::new();
    thesaurus::add_default_synonyms(&mut thesaurus);

    match &cli.command {
        Commands::Remind { input } => remind::execute(input),
        Commands::Note { input } => note::execute(input),
        Commands::Add { input } => add::execute(input),
        Commands::Create { input } => create::execute(input),
        Commands::Show { input } => show::execute(input)
    }
}
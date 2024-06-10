mod remind;
mod note;
mod add;
mod create;
mod show;
mod thesaurus;
mod tokenizer;
mod parser;
mod parse_table;

use clap::{Parser, Subcommand};
use parse_table::{ParseTable, TableRule};
use parser::Rule;
use tokenizer::{TokenDefinition, Tokenizer};

#[derive(Parser)]
#[command(name = "Hey!")]
#[command(about = "Hey is a to quickly write down your thoughts", long_about = None)]
struct Cli {
	// #[command(subcommand)]
	// command: Commands,
	input: Vec<String>
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

// #[derive(Subcommand)]
// enum Commands {
// 	Remind { input: Vec<String> },
// 	Note { input: Vec<String> },
// 	Add { input: Vec<String> },
// 	Create { input: Vec<String> },
// 	Show { input: Vec<String> }
// }

fn main() {
	let cli = Cli::parse();

	let mut thesaurus = thesaurus::Thesaurus::new();
	thesaurus::add_default_synonyms(&mut thesaurus);

	let token_definitions = vec![
		TokenDefinition::new("CLI", vec!["hey"]),
		TokenDefinition::new("COMMAND", vec!["remind", "create"]),
		TokenDefinition::new("SUBJECT", vec!["me", "us", "them"]),
		TokenDefinition::new("KEYWORD", vec!["at", "to", "in"]),
		TokenDefinition::new_with_regex("NUMBER", regex::Regex::new(r"\d+").unwrap()),
		TokenDefinition::new_with_regex("TIME", regex::Regex::new(r"\d+:\d+").unwrap()),
		TokenDefinition::new_with_regex("RELATIVE_TIME", regex::Regex::new(r"\d+:\d+").unwrap()),
		TokenDefinition::new_with_regex("COUNTABLE_TIME", regex::Regex::new(r"\d+:\d+").unwrap()),
		TokenDefinition::new("WORD", vec!["*"]),
		];
	let tokenizer = Tokenizer::new(token_definitions);

	let tokens = tokenizer.tokenize(cli.input.join(" "));

	let rules = vec![
		Rule::new("ReminderCommand", vec!["ReminderNode ContentNode TimeNode".to_string()]),
		Rule::new("ReminderNode", vec!["COMMAND(remind) SUBJECT KEYWORD(to)".to_string()]),
		Rule::new("TimeNode", vec!["KEYWORD(at) NUMBER".to_string()]),
		Rule::new("ContentNode", vec![
			"ContentNode GenNode0".to_string(),
			"GenNode0".to_string()]),
		Rule::new("GenNode0", vec!["SUBJECT".to_string(), "COMMAND".to_string(), "KEYWORD".to_string(), "WORD".to_string()])
	];

	// let parse_table = ParseTable::new(vec![
	// 	TableRule::new("E".to_string(), vec!["A".to_string(), "B".to_string()]),
	// 	TableRule::new("A".to_string(), vec!["WORD(a)".to_string(), "WORD(b)".to_string()]),
	// 	TableRule::new("B".to_string(), vec!["B".to_string(), "B".to_string()]),
	// 	TableRule::new("B".to_string(), vec!["WORD(a)".to_string()]),
	// 	TableRule::new("B".to_string(), vec!["WORD(b)".to_string()]),
	// 	]);
	let parse_table = ParseTable::new(vec![
		TableRule::new("E".to_string(), vec!["E".to_string(), "WORD(*)".to_string(), "B".to_string()]),
		TableRule::new("E".to_string(), vec!["E".to_string(), "WORD(+)".to_string(), "B".to_string()]),
		TableRule::new("E".to_string(), vec!["B".to_string()]),
		TableRule::new("B".to_string(), vec!["WORD(0)".to_string()]),
		TableRule::new("B".to_string(), vec!["WORD(1)".to_string()]),
		]);
	parse_table.print_table();

	// let mut parser = parser::Parser::new(rules);

	// if let Some(tokens) = tokens {
	// 	// for token in &tokens {
	// 	// 	println!("{}(\"{}\")", token.name, token.value);
	// 	// }

	// 	match parser.parse(tokens.as_ref()) {
	// 		Ok(node) => print_tree(&node),
	// 		Err(err) => {
	// 			match err {
	// 					parser::ParseError::UnexpectedToken(val) => println!("{val}"),
	// 					parser::ParseError::UnexpectedEndOfInput(stack) => println!("Parser reached eof without reducing everything. Stack at error: \n{}", stack),
	// 				}
	// 		},
	// 	}
	// }

	// match &cli.command {
	// 	Commands::Remind { input } => remind::execute(input),
	// 	Commands::Note { input } => note::execute(input),
	// 	Commands::Add { input } => add::execute(input),
	// 	Commands::Create { input } => create::execute(input),
	// 	Commands::Show { input } => show::execute(input)
	// }
}

fn print_tree(node: &parser::Node) {
	println!("AST:");
	print_node(node, 1);
}

fn print_node(node: &parser::Node, depth: usize) {
	println!("{: >depth$}Node: {}", " ", node.name);
	println!("{: >depth$}Children:", " ");
	for child in &node.children {
		print_symbol(child, depth + 1)
	}
}

fn print_symbol(symbol: &parser::Symbol, depth: usize) {
	match symbol {
		parser::Symbol::Terminal(leaf) => {
			println!("{: >depth$}{}({})", " ", leaf.name, leaf.value);
		},
		parser::Symbol::NonTerminal(node) => print_node(&node, depth + 1),
	}
}
use crate::{parse_table::{Action, ParseTable, TableRule}, tokenizer::Token};

pub struct Rule {
	name: String,
	rule: String
}

impl Rule {
	pub fn new(name: String, rule: String) -> Self {
		Rule {
			name: name,
			rule: rule
		}
	}
}

pub struct Node {
	name: String,
	children: Vec<Symbol>
}

pub enum Symbol {
	NonTerminal(Node),
	Terminal(String)
}

pub struct Parser {
	parse_table: ParseTable
}

impl Parser {
	pub fn new(rules: Vec<Rule>) -> Self {
		Parser {
			parse_table: ParseTable::new(Self::transform_rules(rules))
		}
	}

	pub fn parse(&self, input: Vec<Token>) -> Option<Node> {
		let mut current_input_idx = 0;
		let mut stack: Vec<Symbol> = Vec::new();

		// while self.parse_table.get_action(0, input.get(current_input_idx)?.) != Action::Accept {

		// }

		None
	}



	fn transform_rules(rules: Vec<Rule>) -> Vec<TableRule> {
		let mut table_rules = Vec::new();
		for rule in rules {
			table_rules.append(&mut Self::adapt_rule(rule));
		}

		return table_rules
	}

	fn adapt_rule(rule: Rule) -> Vec<TableRule> {
		// TODO: Add expansion of "( )", "+", and "*" as MVP
		return vec![
			TableRule::new(
				rule.name,
				rule.rule
					.split(' ')
					.map(|x| x.to_string())
					.collect()
				)
			]
	}
}
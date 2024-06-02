use crate::tokenizer::Token;


pub struct Rule {
	name: String,
	patterns: Vec<String>
}

impl Rule {
	pub fn new(name: &str, patterns: Vec<String>) -> Self {
		Rule {
			name: name.to_string(),
			patterns: patterns
		}
	}
}

#[derive(Debug, Clone)]
pub struct Node {
	pub name: String,
	pub children: Vec<Symbol>
}

#[derive(Debug, PartialEq)]
pub enum Match {
    Full(usize),
    Partial(usize),
	None
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEndOfInput,
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Terminal(Token),
    NonTerminal(Box<Node>),
}

pub struct Parser {
	rules: Vec<Rule>,
	stack: Vec<Symbol>,
	current_pos: usize
}

impl Parser {
	pub fn new(rules: Vec<Rule>) -> Self {
		Parser {
			rules: rules,
			stack: Vec::new(),
			current_pos: 0
		}
	}

	pub fn parse(&mut self, tokens: &Vec<Token>) -> Result<Node, ParseError> {
		while self.current_pos < tokens.len() {
			if let Some(token) = self.lookahead(tokens) {
				self.shift(token.clone());
			}

			println!("Stack before reduction, after shift");
			self.print_stack();

			while self.reduce(tokens) {}
		}

		if self.stack.len() == 1 {
			if let Symbol::NonTerminal(node) = self.stack.pop().unwrap() {
				return Ok(*node);
			}
		}

		Err(ParseError::UnexpectedEndOfInput)
	}

	fn shift(&mut self, token: Token) {
		self.stack.push(Symbol::Terminal(token));
		self.current_pos += 1;
	}

	fn lookahead<'a>(&self, tokens: &'a Vec<Token>) -> Option<&'a Token> {
		tokens.get(self.current_pos)
	}

	fn get_as_token(&self, token_symbol: &str) -> Token {
		if token_symbol.contains('(') {
			let parts = token_symbol.split('(');
			let parts_vec: Vec<&str> = parts.collect();
			return Token{
				name: parts_vec.first().unwrap().to_string(),
				value: parts_vec.last().unwrap().trim_end_matches(')').to_string()
			};
		} else {
			Token{name: token_symbol.to_string(), value: "".to_string()}
		}
	}

	fn reduce(&mut self, tokens: &Vec<Token>) -> bool {
		// If lookahead is part of match, do not reduce
		if let Some(lookahead) = self.lookahead(tokens) {
			if self.is_lookahead_part_of_match(lookahead) {
				println!("Lookahead {} is part of match", lookahead.name);
				return false;
			}
		}

		// Else we can check if a rule is possible to use to reduce
		if let Some((rule_name, full_match_size)) = self.get_full_match_size() {
			println!("Found stack match {}", rule_name);
			let matched_symbols = self.stack.split_off(self.stack.len() - full_match_size);
			self.stack.push(Symbol::NonTerminal(Box::new(Node{
				name: rule_name.to_string(),
				children: matched_symbols
			})));

			println!("Stack after reduction:");
			self.print_stack();

			return true;
		}

		false
	}

	fn print_stack(&self) {
		println!("Stack start: ----");
		for symbol in &self.stack {
			match symbol {
				Symbol::Terminal(token) => println!("Token: {}, value: {}", token.name, token.value),
				Symbol::NonTerminal(node) => println!("Node: {}", node.name),
			}
		}
		println!("Stack end: -----");
	}

	fn create_node(&self, name: &str, matched_symbols: Vec<Symbol>) -> Box<Node> {
		Box::new(Node{
			name: name.to_string(),
			children: matched_symbols
		})
	}

	fn is_lookahead_part_of_match(&self, lookahead: &Token) -> bool {
		// Go through rules, and patterns within the rules
		// For each pattern check if the stack + lookahead results in a partial of full match
		// If any match then return true, else continue looking
		// if nothing retuns true, false

		for rule in &self.rules {
			for pattern in &rule.patterns {
				let partial_stack_size = match self.match_pattern_with_stack(pattern) {
					Match::Partial(size) => Some(size),
					Match::Full(_) | Match::None => None,
				};

				if let Some(partial_stack_size) = partial_stack_size {
					let pattern_split: Vec<&str> = pattern.split(' ').collect();
					if let Some(pattern_symbol) = pattern_split.get(partial_stack_size) {
						let pattern_token = self.get_as_token(&pattern_symbol);
						if pattern_token.name == lookahead.name && (pattern_token.value == lookahead.value || pattern_token.value.is_empty()) {
							return true;
						}
					}
				}
			}
		}

		false
	}

	fn get_full_match_size(&self) -> Option<(String, usize)> {
		for rule in &self.rules {
			for pattern in &rule.patterns {
				match self.match_pattern_with_stack(pattern) {
					Match::Full(size) => return Some((rule.name.to_string(), size)),
					Match::Partial(_) | Match::None => (),
				};
			}
		}

		None
	}

	fn match_pattern_with_stack(&self, pattern: &str) -> Match {
		for slice_size in 1..self.stack.len() + 1 {
			let stack_idx = self.stack.len() - slice_size;
			let pattern_symbols = pattern.split(' ');
			let mut pattern_match = false;

			for (pattern_idx, pattern_str) in pattern_symbols.enumerate() {
				if pattern_idx >= slice_size {
					if pattern_match {
						println!("Found partial match for {}, with slice size: {}", pattern, slice_size);
						return Match::Partial(slice_size);
					}

					break; // Might be able to move this to the else clause instead
				}

				if let Some(stack_symbol) = self.stack.get(stack_idx + pattern_idx) {
					// If pattern_str isn't a token it will populate a token with a name and empty value - consider changing name and/or functions
					let pattern_token = self.get_as_token(pattern_str);

					pattern_match = match stack_symbol {
							Symbol::Terminal(stack_token) => stack_token.name == pattern_token.name && (stack_token.value == pattern_token.value || pattern_token.value.is_empty()),
							Symbol::NonTerminal(stack_node) => stack_node.name == pattern_token.name,
						};
				} else {
					panic!("How did we get here?");
				}
			}

			if pattern_match {
				return Match::Full(slice_size);
			}
		}

		Match::None
	}
}
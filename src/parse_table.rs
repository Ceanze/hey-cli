use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub enum ParseTableError {
	NoRulesSpecified,
	InvalidTableRule,
	InvalidSymbolForSet,
	InvalidSetIndex
}

pub enum Action<'a> {
	Shift(usize),
	Reduce(&'a TableRule),
	Error(ParseTableError)
}

#[derive(PartialEq, Debug, Clone)]
struct Set {
	index: usize,
	rules: Vec<TableRule>
}

impl Set {
	pub fn is_same_kernel(&self, other: &Set) -> bool {
		let longest_rules = if self.rules.len() < other.rules.len() { &other.rules } else { &self.rules };
		let shortest_rules = if self.rules.len() < other.rules.len() { &self.rules } else { &other.rules };

		for rule1 in longest_rules {
			if !rule1.is_kernel_rule {
				continue;
			}

			let mut found_matching_rule = false;
			for rule2 in shortest_rules {
				if rule2.is_kernel_rule
					&& rule1.left == rule2.left
					&& rule1.next_symbol_index == rule2.next_symbol_index
					&& rule1.right == rule2.right {
					found_matching_rule = true;
				}
			}

			if !found_matching_rule {
				return false;
			}
		}

		return true;
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct TableRule {
	left: String,
	right: Vec<String>,
	next_symbol_index: usize,
	next_set_index: Option<usize>,
	is_kernel_rule: bool
}

impl TableRule {
	pub fn new(left: String, right: Vec<String>) -> Self {
		TableRule {
			left: left,
			right: right,
			next_symbol_index: 0,
			next_set_index: None,
			is_kernel_rule: true
		}
	}

	pub fn clone_and_advance(&self) -> TableRule {
		let mut new_rule = self.clone();
		new_rule.next_symbol_index = self.next_symbol_index + 1;
		new_rule.next_set_index = None;
		new_rule.is_kernel_rule = true;
		return new_rule
	}

	pub fn clone_as_expanded_rule(&self) -> TableRule {
		let mut new_rule = self.clone();
		new_rule.is_kernel_rule = false;
		return new_rule
	}

	pub fn can_advance(&self) -> bool {
		self.next_symbol_index < self.right.len() && self.right.get(self.next_symbol_index).unwrap() != "$"
	}

	pub fn can_expand(&self) -> bool {
		self.next_symbol_index < self.right.len()
	}

	pub fn is_reducable(&self) -> bool {
		self.next_symbol_index == self.right.len()
	}

	pub fn matches_lookahead(&self, lookahead: &str) -> bool {
		if let Some(symbol) = self.right.get(self.next_symbol_index) {
			return symbol == lookahead;
		}

		false
	}
}

pub struct ParseTable {
	rules: Vec<TableRule>,
	sets: Vec<Set>,
	possible_reductions: HashMap<usize, TableRule>
}

impl ParseTable {
	pub fn new(rules: Vec<TableRule>) -> Self {
		let mut parse_table = ParseTable {
			rules: rules,
			sets: Vec::new(),
			possible_reductions: HashMap::new()
		};

		parse_table.construct().unwrap();
		return parse_table;
	}

	pub fn get_action(&self, set_index: usize, symbol: &str) -> Action {
		if let Some(set) = self.sets.get(set_index) {
			for rule in &set.rules {
				if rule.matches_lookahead(symbol) {
					return Action::Shift(rule.next_set_index.unwrap());
				}
			}

			if let Some(rule) = self.possible_reductions.get(&set_index) {
				return Action::Reduce(rule);
			}

			return Action::Error(ParseTableError::InvalidSymbolForSet);
		}

		Action::Error(ParseTableError::InvalidSetIndex)
	}

	pub fn print_table(&self) {
		for set in &self.sets {
			println!("\nSet {}:", set.index);
			for rule in &set.rules {
				print!("  {} -> ", rule.left);
				let mut placed_dot = false;
				for (idx, right) in rule.right.iter().enumerate() {
					if rule.next_symbol_index == idx {
						print!("*");
						placed_dot = true;
					}
					print!(" {}", right);
				}
				if !placed_dot {
					print!("*");
				}
				print!("  (S{})", rule.next_set_index.unwrap_or(99999));
				print!("  (Kernel: {:?})", rule.is_kernel_rule);

				println!("")
			}
		}
	}

	fn construct(&mut self) -> Result<(), ParseTableError> {
		let start_rule = self.add_start_rule()?;

		// Add the first set as starting point
		let mut set_queue = VecDeque::new();
		set_queue.push_back(Set{index: 0, rules: vec![start_rule.clone()]});

		let mut set_index: usize = 0;
		while !set_queue.is_empty() {
			let set = set_queue.pop_front().unwrap();

			let mut full_set = self.expand_set(set)?;
			let new_sets = self.get_advanced_sets(&mut full_set, set_index + set_queue.len() + 1, &set_queue);
			set_queue.extend(new_sets);
			self.sets.push(full_set);
			set_index += 1;
		}

		self.populate_reductions();

		Ok(())
	}

	fn add_start_rule(&mut self) -> Result<&TableRule, ParseTableError> {
		// Right now we assume the first rule specifed is the start rule
		let start_rule = self.rules.first().ok_or(ParseTableError::NoRulesSpecified)?;

		// Construct a new start rule with syntax S -> start_rule eof to remove any ambiguity
		self.rules.push(TableRule::new("StartNode".to_string(), vec![start_rule.left.clone(), "$".to_string()]));


		Ok(self.rules.last().unwrap())
	}

	fn expand_set(&self, mut set: Set) -> Result<Set, ParseTableError> {
		let mut rule_idx = 0;
		while rule_idx < set.rules.len() {
			let rule = set.rules.get(rule_idx).unwrap();

			if !rule.can_expand() {
				rule_idx += 1;
				continue;
			}

			let left = rule.right.get(rule.next_symbol_index).ok_or(ParseTableError::InvalidTableRule)?.clone(); // TODO: Check if this can be done without a clone but still follow the borrow checkers rules

			for initial_rule in &self.rules {
				// TODO: Check if in set in a better way to avoid .Clone before we actually need it
				let new_rule = initial_rule.clone_as_expanded_rule();
				if initial_rule.left == *left && !set.rules.contains(&new_rule) {
					set.rules.push(new_rule);
				}
			}

			rule_idx += 1;
		}

		Ok(set)
	}

	fn get_advanced_sets(&self, set: &mut Set, mut next_free_set_index: usize, set_queue: &VecDeque<Set>) -> Vec<Set> {
		let mut advanced_sets: Vec<Set> = Vec::new();

		for rule in &mut set.rules {
			// Only check rules which has something to advance
			if !rule.can_advance() {
				continue;
			}

			// Check if a set with the "DOT Symbol" has already been added, add to that set if so
			let mut added_to_set = false;
			for advanced_set in &mut advanced_sets {
				let advanced_rule = advanced_set.rules.first().unwrap();
				if advanced_rule.right.get(advanced_rule.next_symbol_index - 1).unwrap() == rule.right.get(rule.next_symbol_index).unwrap() {
					advanced_set.rules.push(rule.clone_and_advance());
					rule.next_set_index = Some(advanced_set.index);
					added_to_set = true;
					break;
				}
			}

			if !added_to_set {
				advanced_sets.push(Set{index: next_free_set_index, rules: vec![rule.clone_and_advance()]});
				rule.next_set_index = Some(next_free_set_index);
				next_free_set_index += 1;
			}
		}

		let filtered_sets = advanced_sets
			.iter()
			.filter(|new_set| {
				!self.sets.iter().any(|s| s.is_same_kernel(&new_set))
				&& !set_queue.iter().any(|s| s.is_same_kernel(&new_set))
				&& !set.is_same_kernel(&new_set)})
			.cloned();

		filtered_sets.collect()
	}

	fn populate_reductions(&mut self) {
		for set in &self.sets {
			for rule in &set.rules {
				if rule.is_reducable() {
					self.possible_reductions.insert(set.index, rule.clone());
					break;
				}
			}
		}
	}
}
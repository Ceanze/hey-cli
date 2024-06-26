/*
	Defines a token that will be constructed by the tokenizer
	Name should be in SCREAMING_SNAKE_CASE
	Words contain a list of words that define the token. Wildcard token can be created using an "*".
		This token then consumes all words that don't match any other token.
*/
pub struct TokenDefinition {
	name: String,
	words: Option<Vec<String>>,
	regex: Option<regex::Regex>
}

impl TokenDefinition {
	pub fn new(name: &str, words: Vec<&str>) -> Self {
		TokenDefinition {
			name: name.to_string(),
			words: Some(words.iter().map(|w| w.to_string()).collect()),
			regex: None
		}
	}

	pub fn new_with_regex(name: &str, pattern: regex::Regex) -> Self {
		TokenDefinition {
			name: name.to_string(),
			words: None,
			regex: Some(pattern)
		}
	}
}

#[derive(Debug, Clone)]
pub struct Token {
	pub name: String,
	pub value: String
}

pub struct Tokenizer {
	token_definitions: Vec<TokenDefinition>
}

impl Tokenizer {
	pub fn new(token_definitions: Vec<TokenDefinition>) -> Self {
		Tokenizer {
			token_definitions: token_definitions
		}
	}

	pub fn tokenize<Str>(&self, str: Str) -> Option<Vec<Token>>
	where Str: AsRef<str> {
		let mut tokens: Vec<Token> = Vec::new();
		let wildcard_token_def = self.get_wildcard_token_definition();

		if str.as_ref().is_empty() {
			return None;
		}

		for word in str.as_ref().split(' ') {
			let mut tokenized = false;
			for token_definition in &self.token_definitions {
				if self.is_word_match(word, &token_definition) {
					tokenized = true;
					tokens.push(Token{
						name: token_definition.name.clone(),
						value: word.to_string()
					});
				}
			}

			if !tokenized {
				if let Some(wildcard_token_def) = wildcard_token_def {
					tokens.push(Token{
						name: wildcard_token_def.name.clone(),
						value: word.to_string()
					})
				} else {
					println!("[Error] Found word '{}' which doesn't match any token, and there is no wildcard token available", word);
					return None;
				}
			}
		}

		Some(tokens)
	}

	fn is_word_match(&self, word: &str, token_definition: &TokenDefinition) -> bool {
		if let Some(pattern) = &token_definition.regex {
			if pattern.is_match(word) {
				return true;
			}
			return false;
		} else if let Some(words) = &token_definition.words {
			for pattern_word in words {
				if word == pattern_word {
					return true;
				}
			}
			return false;
		}

		panic!("[Error] Token definition '{}' did not have words nor regex to match", token_definition.name);
	}

	fn get_wildcard_token_definition(&self) -> Option<&TokenDefinition> {
		let mut definition: Option<&TokenDefinition> = None;
		for token_definition in &self.token_definitions {
			if let Some(words) = &token_definition.words {
				for word in words {
					if word == "*" && definition.is_none() {
						definition = Some(&token_definition);
					} else if word == "*" && definition.is_some() {
						println!("[Warn] More than one defintion has been added for the wildcard '*'. This is not allowed and can cause unintended side effects.");
						println!("[Warn] Already found TokenDefinition {}, also found {}", definition.unwrap().name, token_definition.name);
						return None;
					}
				}
			}
		}

		return definition;
	}
}
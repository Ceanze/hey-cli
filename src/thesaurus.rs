use std::collections::HashMap;

pub struct Thesaurus {
	synonyms: HashMap<String, Vec<String>>
}

impl Thesaurus {
	pub fn new() -> Self {
		Thesaurus {
			synonyms: HashMap::new(),
		}
	}

	pub fn add_synonyms(&mut self, word: &str, synonyms: Vec<&str>) {
		self.synonyms.insert(word.to_string(), synonyms.iter().map(|s| s.to_string()).collect());
	}

	pub fn is_synonym(&self, word: &str, synonym: &str) -> bool {
        if let Some(synonyms) = self.synonyms.get(word) {
            synonyms.contains(&synonym.to_string())
        } else {
            false
        }
	}

	pub fn get_synonyms(&self, word: &str) -> &Vec<String> {
		self.synonyms.get(word).unwrap()
	}
}

pub fn add_default_synonyms(thesaurus: &mut Thesaurus) {
	thesaurus.add_synonyms("called", vec!["named"]);
    thesaurus.add_synonyms("create", vec!["construct", "build", "devise", "design", "establish", "forge", "form", "generate", "initiate", "invent", "make", "produce", "set up", "spawn"]);
}
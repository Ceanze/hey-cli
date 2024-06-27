
// hey remind me to X at Y -> Tokenized: CLI: "hey", COMMAND: "remind", SUBJECT: "me", KEYWORD: "to", WORD: "X"..., KEYWORD: "at", TIME: "y"

// Variations:
// "hey remind me to X at Y"				CLI COMMAND SUBJECT TO CONTENT AT TIME
// "hey remind us to X at Y"				CLI COMMAND SUBJECT TO CONTENT AT TIME
// "hey remind me at Y to X"				CLI COMMAND SUBJECT AT TIME TO CONTENT
// "hey remind me to X tomorrow at Y"		CLI COMMAND SUBJECT TO CONTENT RELATIVE_DAY AT TIME
// "hey remind me to X in two weeks"		CLI COMMAND SUBJECT TO CONTENT IN NUMBER COUNTABLE_DAY
// "hey remind me on monday at 5pm to X"	CLI COMMAND SUBJECT ON DAY AT TIME TO CONTENT
// "hey remind me to X on monday"			CLI COMMAND SUBJECT TO CONTENT ON DAY

// Variations where it doesn't start with "remind" (for free text option)
// "hey in two weeks remind me to X"		CLI IN NUMBER COUNTABLE_DAY COMMAND SUBJECT TO CONTENT
// "hey at 5pm remind me to X"				CLI AT TIME COMMAND SUBJECT TO CONTENT

// "hey add to list to remind me to eat pasta"

use clap::Args;

#[derive(Args)]
pub struct Command {
	free_text: Vec<String>
}

pub fn execute(input: Command) -> anyhow::Result<()> {

	Ok(())
}
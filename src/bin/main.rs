
use kgemu::{compile, definitions::language, prelude::*};

fn main() {
	

	let thumb_def: language::LanguageDefinition = Default::default();
	let mut parsed_simple: compile::parse_code::ParsedCode = Default::default();
	parsed_simple.parse_from_file("./sample_assembly_code/command_tests.thumb", thumb_def);

	println!("File Name: {}", parsed_simple.file_name);
	println!("File Length: {}", parsed_simple.file_size);
	println!("Contents:");

	for (i, line) in parsed_simple.lines.iter().enumerate() {
		print!("{}:", i);
		for section in &line.sections {
			print!(" {:?} ", section)
		}
		println!("");
	}

	for c in parsed_simple.commands {
		println!("{:?}", c);
	}

	println!("End of file");
}
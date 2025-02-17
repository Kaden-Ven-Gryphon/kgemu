
use kgemu::{compile, definitions::{self, language}, prelude::*};

fn main() {
	hello_compile();
	hello_emulate();

	let thumb_def: language::LanguageDefinition = Default::default();
	let parsed_simple = compile::parse_code::parse_from_file("./sample_assembly_code/simple.thumb", thumb_def);

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

	println!("End of file");
}
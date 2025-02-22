use std::{str::FromStr, fs};


use regex::{Regex, SetMatches};

use crate::definitions::language;

#[derive(Default, Debug)]
pub enum SectionType {
	#[default]
	Comment,
	Label,
	OpCode,
	Command,
	Operand,
	ComplierMark,
	Literal,
}

impl FromStr for SectionType {
	type Err = String;

	fn from_str(s: &str) -> Result<SectionType, String> {
		match s {
			"comment" => Ok(SectionType::Comment),
			"label" => Ok(SectionType::Label),
			"opc" => Ok(SectionType::OpCode),
			"compliemark" => Ok(SectionType::ComplierMark),
			"literal" => Ok(SectionType::Literal),
			"command" => Ok(SectionType::Command),
			_ => {
				if s.starts_with("op") && s.ends_with(&['0','1','2','3','4','5','6','7','8','9']) {
					Ok(SectionType::Operand)
				}
				else {
					Err(format!("\"{}\": is an unknown type for SectionType",s))
				}
			},
		}
	}
}


#[derive(Default, Debug)]
pub struct ParsedLine {
	pub sections: Vec<(SectionType, String)>

}

#[derive(Default)]
pub struct ParsedCode {
	pub file_name: String,
	pub file_size: i32,
	pub lines: Vec<ParsedLine>
}

impl language::LanguageDefinition {
	pub fn parse_line(&self, line: &str) -> ParsedLine {
		let mut parsed_line: ParsedLine = Default::default();

		for re in &self.regex_list {
			let regex = Regex::new(re.as_str()).unwrap();

			if let Some(caps) = regex.captures(line) {
				for n in regex.capture_names().into_iter() {
					match n {
						None => {}
						Some(name) => {
							if let Some(cap) = &caps.name(name) {
								let section = (SectionType::from_str(name).unwrap(), cap.as_str().to_string());
								parsed_line.sections.push(section);
								//print!("{}: {} | ", name, cap.as_str());
							}
						}
					}
				}
			}
		}

		parsed_line
	}

	pub fn parse_from_file(&self, file_name: &str) -> ParsedCode {
	let mut parsed_code: ParsedCode = Default::default();
	
	parsed_code.file_name = file_name.to_string();

	// Get the contents from a filer	
	let contents = fs::read_to_string(file_name)
		.expect("Could not open file.");

	for line in contents.lines() {
		let new_line = self.parse_line(line);
		parsed_code.lines.push(new_line);
	}

	parsed_code
}
}

/// takes a line and uses a def regex to parse into parts
pub fn parse_line(line: &str, def: &language::LanguageDefinition) -> ParsedLine {
	let mut parsed_line: ParsedLine = Default::default();

	

	let re = def.line_regex();
	if let Some(caps) = re.captures(line)
	{
		for names in re.capture_names().into_iter() {
			match names {
				None => {}
				Some(name) => {
					if let Some(cap) = &caps.name(name) {
						let section = (SectionType::from_str(name).unwrap(), cap.as_str().to_string());
						parsed_line.sections.push(section);
						//print!("{}: {} | ", name, cap.as_str());
					}
				}
			}
		}
	}
	
	

	parsed_line
}

pub fn parse_from_file(file_name: &str, def: language::LanguageDefinition ) -> ParsedCode {
	let mut parsed_code: ParsedCode = Default::default();
	
	parsed_code.file_name = file_name.to_string();

	// Get the contents from a filer	
	let contents = fs::read_to_string(file_name)
		.expect("Could not open file.");

	for line in contents.lines() {
		let new_line = parse_line(line, &def);
		parsed_code.lines.push(new_line);
	}

	parsed_code
}
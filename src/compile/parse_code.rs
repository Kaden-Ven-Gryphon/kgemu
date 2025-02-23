use std::{str::FromStr, fs};


use regex::Regex;

use crate::{definitions::{language, processor}, prelude::SegType};

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
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
pub struct ParsedCommand {
	pub op_code: String,
	pub address: i32,
	pub format: i32,
	pub operands: Vec<(processor::SegType, String)>,
}

#[derive(Default, Debug)]
pub struct ParsedLabel {
	pub name: String,
	pub address: i32,
}

#[derive(Default, Debug)]
pub struct ParsedVar {
	pub label: String,
	pub address: i32,
	pub size: i32,
	pub value: String,
}


#[derive(Default, Debug)]
pub struct ParsedLine {
	pub sections: Vec<(SectionType, String)>,
	pub index: i32,
}

#[derive(Default)]
pub struct ParsedCode {
	pub file_name: String,
	pub file_size: i32,
	pub lines: Vec<ParsedLine>,
	pub labels: Vec<ParsedLabel>,
	pub vars: Vec<ParsedVar>,
	pub commands: Vec<ParsedCommand>,
}

impl ParsedCode {
	/// takes a line and uses a def regex to parse into parts
	pub fn parse_line(&self, line: &str, def: &language::LanguageDefinition) -> ParsedLine {
		let mut parsed_line: ParsedLine = Default::default();

			for re in &def.regex_list {
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

	pub fn parse_command(&self, command: String, def: &language::LanguageDefinition ) -> Option<ParsedCommand> {
		

		// For each command word
		for cmd in &def.commands {
			// For each version of the command
			for cmd_version in &cmd.1 {
				// Get the regex for that version of the command
				let re = Regex::new(cmd_version.regex.as_str()).unwrap();
				// If the command fits the regex start parsing the captures
				if let Some(caps) = re.captures(&command) {
					let mut parsed_command: ParsedCommand = Default::default();
					parsed_command.op_code = cmd.0.clone();
					parsed_command.format = cmd_version.format_index;
					for seg in &cmd_version.segments {
						match seg.0 {
							SegType::Op => {
								parsed_command.operands.push((seg.0,seg.1.clone()));
							}
							SegType::Flag => {
								parsed_command.operands.push((seg.0,seg.1.clone()));
							}
							_ => {
								if let Some(cap) = caps.name(seg.1.as_str()) {
									parsed_command.operands.push((seg.0, cap.as_str().to_string()));
								}
							}
						}
					}
					return Some(parsed_command);
				}
			}
		}
		None
	}

	pub fn parse_from_file(&mut self, file_name: &str, def: language::LanguageDefinition )  {
		
		
		self.file_name = file_name.to_string();

		// Get the contents from a filer	
		let contents = fs::read_to_string(file_name)
			.expect("Could not open file.");

		for line in contents.lines() {
			let new_line = self.parse_line(line, &def);

			
			self.lines.push(new_line);

			let new_line =  self.lines.last().unwrap();
			
			let command = new_line.sections.iter().find(|i| i.0 == SectionType::Command);
			let label = new_line.sections.iter().find(|i| i.0 == SectionType::Label);
			let comp_mark = new_line.sections.iter().find(|i| i.0 == SectionType::ComplierMark);
			let literal = new_line.sections.iter().find(|i| i.0 == SectionType::Literal);

			match command {
				None => {}
				Some(c) => {
					let new_command = self.parse_command(c.1.clone(), &def);
					match new_command {
						None => {}
						Some(s) => {self.commands.push(s);}
					}
					
				}
			}
		}
	}
}

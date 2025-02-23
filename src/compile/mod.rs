/// Mod to compile assembly code into machine code
use crate::prelude::LanguageDefinition;


pub mod parse_code;

pub mod prelude {
	pub use super::Complier;
}

pub struct Complier {
	language_def: LanguageDefinition,
	parsed_code: parse_code::ParsedCode,
	complied_code: Vec<u8>
}

impl Complier {
	pub fn load_def_from_file() {

	}
	pub fn set_def() {
		
	}
	pub fn load_from_file() {

	}
	pub fn load_from_str() {

	}
	pub fn parse() {

	}
	pub fn parse_from_file() {

	}
	pub fn parse_from_str() {

	}
	pub fn compile() {

	}
	pub fn compile_from_file() {

	}
	pub fn compile_from_str() {

	}
	pub fn get_bin() {

	}
	pub fn get_bin_as_hex() {

	}
	pub fn get_bin_as_bin() {

	}
}
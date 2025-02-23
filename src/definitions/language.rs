use super::processor::{ProcessorDefinition, SegType};


pub mod prelude {
	pub use super::CommandDefinition;
	pub use super::LanguageDefinition;
}

// This is a way to organize a set of capture groups for generating regex exspresions to parse code
// Not fully implemented yet

/* 
struct VecTree<T> {
	pub value: Option<T>,
	pub adjacent: Option<Box<VecTree<T>>>,
	pub right: Option<Box<VecTree<T>>>,
	pub left: Option<Box<VecTree<T>>>,
}

// used to create the regex, not implemented yet
enum CaptureGroupWSBuffer {
	Before,
	After,
	Both,
	None,
}

struct CaptureGroup {
	pub name: String,
	pub regex: String,
	pub capture: bool,
	pub ws_buffer: CaptureGroupWSBuffer,
}

 */

pub struct CommandDefinition {
	pub regex: String,
	pub segments: Vec<(SegType, String)>,
	pub format_index: i32,
}

pub struct LanguageDefinition { 
	pub processor_def: ProcessorDefinition,
	pub regex_list: Vec<String>,
	//pub capture_groups:Vec<VecTree<CaptureGroup>>,
	pub commands: Vec<(String,Vec<CommandDefinition>)>,
}
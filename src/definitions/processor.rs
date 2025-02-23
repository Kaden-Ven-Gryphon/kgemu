pub mod prelude {
	pub use super::SegType;
	pub use super::OperationSeg;
	pub use super::Format;
	pub use super::ProcessorDefinition;
}

#[derive(Default, Debug, Clone, Copy)]
pub enum SegType {
	#[default]
	Main,
	Op,
	Flag,
	Source,
	Destination,
	Offset,
	Immediate,
	Condition
}
pub struct OperationSeg {
	pub name: Option<String>,
	pub mask: Vec<u8>,
	pub seg_type: SegType,
	pub values: Option<Vec<Vec<u8>>>
}
pub struct Format {
	pub id: i32,
	pub name: String,
	pub segments: Vec<OperationSeg>,
}

pub struct ProcessorDefinition {
	pub name: String,
	pub num_register: i32,
	pub register_size: i32,
	pub formats: Vec<Format>,
}
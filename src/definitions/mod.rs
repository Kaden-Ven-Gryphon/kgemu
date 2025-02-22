


pub mod prelude {
	use super::language::*;
	use super::processor::*;
	use super::device::*;
}


pub mod language {

	use std::vec;
	use super::processor::{ProcessorDefinition, SegType};

	// This is a way to organize a set of capture groups for generating regex exspresions to parse code
	// Not fully implemented yet
	pub struct VecTree<T> {
		pub value: Option<T>,
		pub adjacent: Option<Box<VecTree<T>>>,
		pub right: Option<Box<VecTree<T>>>,
		pub left: Option<Box<VecTree<T>>>,
	}

	// used to create the regex, not implemented yet
	pub enum CaptureGroupWSBuffer {
		Before,
		After,
		Both,
		None,
	}

	pub struct CaptureGroup {
		pub name: String,
		pub regex: String,
		pub capture: bool,
		pub ws_buffer: CaptureGroupWSBuffer,
	}



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

  
	impl Default for LanguageDefinition {
		fn default() -> Self {
			LanguageDefinition {
				
				processor_def: Default::default(),
				regex_list: vec![
					r"^(?:[ \t]*)(?:(?P<label>[a-zA-Z_][a-zA-Z0-9_]*):)?(?:[ \t]*)(?P<command>[a-zA-Z][a-zA-Z0-9# \t,]*)?(?:[ \t]*)(?P<comment>@.*)?$".to_string(),
					r"^(?:[ \t]*)(?P<compliemark>.[a-zA-Z]*)(?:[ \t]*)(?P<literal>[a-zA-Z_]+)?(?:[ \t]*)(?P<comment>@.*)?$".to_string(),
					r##"^(?:[ \t]*)(?:(?P<label>[a-zA-Z_][a-zA-Z0-9_]*):)(?:[ \t]*)(?P<compliemark>.[a-zA-Z]*)(?:[ \t]*)(?P<literal>[#0-9xbn]+|"[\w\s]*")(?:[ \t]*)(?P<comment>@.*)?$"##.to_string(),
				],
				/*/
				capture_groups:vec![VecTree{
					value: Some(CaptureGroup{
						name: "label_outer".to_string(),
						regex: r":".to_string(),
						capture: false,
						ws_buffer: CaptureGroupWSBuffer::Both,
					}),
					adjacent: None,
					right: Some(Box::new(VecTree{
						value: Some(CaptureGroup {
							name: "label".to_string(),
							regex: r"".to_string(),
							capture: true,
							ws_buffer: CaptureGroupWSBuffer::None,
						}),
						adjacent: None,
						left: None,
						right: None,
					} )),
					left: None,
				},], */
				commands: vec![
					("ADC".to_string(), vec![
						CommandDefinition{
							regex: r"ADC[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"5".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
					("ADD".to_string(), vec![
						CommandDefinition{
							regex: r"ADD[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7]),[ \t]+(?P<immediate>[rR][0-7])".to_string(),
							segments: vec![(SegType::Flag,"0".to_string()),(SegType::Op,"0".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string()),(SegType::Immediate,"immediate".to_string())],
							format_index: 2
						},
						CommandDefinition{
							regex: r"ADD[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7]),[ \t]+(?P<immediate>#0x[0-9a-fA-F]+|#[0-9]+)".to_string(),
							segments: vec![(SegType::Flag,"1".to_string()),(SegType::Op,"0".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string()),(SegType::Immediate,"immediate".to_string())],
							format_index: 2
						},
						CommandDefinition{
							regex: r"ADD[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<offset>#0x[0-9a-fA-F]+|#[0-9]+)".to_string(),
							segments: vec![(SegType::Op,"2".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Immediate,"offset".to_string())],
							format_index: 3
						},
						CommandDefinition{
							regex: r"ADD[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[hH][0-7])".to_string(),
							segments: vec![(SegType::Op,"0".to_string()),(SegType::Flag,"0".to_string()),(SegType::Flag,"1".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 5
						},
						CommandDefinition{
							regex: r"ADD[ \t]+(?P<destination>[hH][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"0".to_string()),(SegType::Flag,"1".to_string()),(SegType::Flag,"0".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 5
						},
						CommandDefinition{
							regex: r"ADD[ \t]+(?P<destination>[hH][0-7]),[ \t]+(?P<source>[hH][0-7])".to_string(),
							segments: vec![(SegType::Op,"0".to_string()),(SegType::Flag,"1".to_string()),(SegType::Flag,"1".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 5
						},
					]),
					("AND".to_string(), vec![
						CommandDefinition{
							regex: r"AND[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"0".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
					("ASR".to_string(), vec![
						CommandDefinition{
							regex: r"ASR[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7]),[ \t]+(?P<offset>#0x[0-9a-fA-F]+|#[0-9]+)".to_string(),
							segments: vec![(SegType::Op,"2".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string()),(SegType::Immediate,"offset".to_string())],
							format_index: 1
						},
						CommandDefinition{
							regex: r"ASR[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"4".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
					("B".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("B".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("B[a][a]".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("BIC".to_string(), vec![
						CommandDefinition{
							regex: r"BIC[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"14".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
					("BL".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("BX".to_string(), vec![
						CommandDefinition{
							regex: r"BX[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"3".to_string()),(SegType::Flag,"0".to_string()),(SegType::Flag,"1".to_string()),(SegType::Source,"source".to_string())],
							format_index: 5
						},
						CommandDefinition{
							regex: r"BX[ \t]+(?P<source>[hH][0-7])".to_string(),
							segments: vec![(SegType::Op,"3".to_string()),(SegType::Flag,"0".to_string()),(SegType::Flag,"1".to_string()),(SegType::Source,"source".to_string())],
							format_index: 5
						},
					]),
					("CMN".to_string(), vec![
						CommandDefinition{
							regex: r"CMN[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"11".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
					("CMP".to_string(), vec![
						CommandDefinition{
							regex: r"CMP[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<offset>#0x[0-9a-fA-F]+|#[0-9]+)".to_string(),
							segments: vec![(SegType::Op,"1".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Immediate,"offset".to_string())],
							format_index: 3
						},
						CommandDefinition{
							regex: r"CMP[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"10".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
						CommandDefinition{
							regex: r"CMP[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[hH][0-7])".to_string(),
							segments: vec![(SegType::Op,"1".to_string()),(SegType::Flag,"0".to_string()),(SegType::Flag,"1".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 5
						},
						CommandDefinition{
							regex: r"CMP[ \t]+(?P<destination>[hH][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"1".to_string()),(SegType::Flag,"1".to_string()),(SegType::Flag,"0".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 5
						},
						CommandDefinition{
							regex: r"CMP[ \t]+(?P<destination>[hH][0-7]),[ \t]+(?P<source>[hH][0-7])".to_string(),
							segments: vec![(SegType::Op,"1".to_string()),(SegType::Flag,"1".to_string()),(SegType::Flag,"1".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 5
						},
					]),
					("EOR".to_string(), vec![
						CommandDefinition{
							regex: r"EOR[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"1".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
					("LDMIA".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("LDR".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("LDRB".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("LDRH".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("LSL".to_string(), vec![
						CommandDefinition{
							regex: r"LSL[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7]),[ \t]+(?P<offset>#0x[0-9a-fA-F]+|#[0-9]+)".to_string(),
							segments: vec![(SegType::Op,"0".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string()),(SegType::Immediate,"offset".to_string())],
							format_index: 1
						},
						CommandDefinition{
							regex: r"LSL[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"2".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
					("LDSB".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("LDSH".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("LSR".to_string(), vec![
						CommandDefinition{
							regex: r"LSR[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7]),[ \t]+(?P<offset>#0x[0-9a-fA-F]+|#[0-9]+)".to_string(),
							segments: vec![(SegType::Op,"1".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string()),(SegType::Immediate,"offset".to_string())],
							format_index: 1
						},
						CommandDefinition{
							regex: r"LSR[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"3".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
					("MOV".to_string(), vec![
						CommandDefinition{
							regex: r"MOV[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<offset>#0x[0-9a-fA-F]+|#[0-9]+)".to_string(),
							segments: vec![(SegType::Op,"0".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Immediate,"offset".to_string())],
							format_index: 3
						},
						CommandDefinition{
							regex: r"MOV[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[hH][0-7])".to_string(),
							segments: vec![(SegType::Op,"2".to_string()),(SegType::Flag,"0".to_string()),(SegType::Flag,"1".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 5
						},
						CommandDefinition{
							regex: r"MOV[ \t]+(?P<destination>[hH][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"2".to_string()),(SegType::Flag,"1".to_string()),(SegType::Flag,"0".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 5
						},
						CommandDefinition{
							regex: r"MOV[ \t]+(?P<destination>[hH][0-7]),[ \t]+(?P<source>[hH][0-7])".to_string(),
							segments: vec![(SegType::Op,"2".to_string()),(SegType::Flag,"1".to_string()),(SegType::Flag,"1".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 5
						},
					]),
					("MUL".to_string(), vec![
						CommandDefinition{
							regex: r"MUL[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"13".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
					("MVN".to_string(), vec![
						CommandDefinition{
							regex: r"MVN[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"15".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
					("NEG".to_string(), vec![
						CommandDefinition{
							regex: r"NEG[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"9".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
					("ORR".to_string(), vec![
						CommandDefinition{
							regex: r"ORR[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"12".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
					("POP".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("PUSH".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("ROR".to_string(), vec![
						CommandDefinition{
							regex: r"ROR[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"7".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
					("SBC".to_string(), vec![
						CommandDefinition{
							regex: r"SBC[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"6".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
					("STMIA".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("STR".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("STRB".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("STRH".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("SWI".to_string(), vec![
						CommandDefinition{
							regex: "".to_string(),
							segments: vec![],
							format_index: 0
						}
					]),
					("SUB".to_string(), vec![
						CommandDefinition{
							regex: r"SUB[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7]),[ \t]+(?P<immediate>[rR][0-7])".to_string(),
							segments: vec![(SegType::Flag,"0".to_string()),(SegType::Op,"1".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string()),(SegType::Immediate,"immediate".to_string())],
							format_index: 2
						},
						CommandDefinition{
							regex: r"SUB[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7]),[ \t]+(?P<immediate>#0x[0-9a-fA-F]+|#[0-9]+)".to_string(),
							segments: vec![(SegType::Flag,"1".to_string()),(SegType::Op,"1".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string()),(SegType::Immediate,"immediate".to_string())],
							format_index: 2
						},
						CommandDefinition{
							regex: r"SUB[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<offset>#0x[0-9a-fA-F]+|#[0-9]+)".to_string(),
							segments: vec![(SegType::Op,"3".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Immediate,"offset".to_string())],
							format_index: 3
						}
					]),
					("TST".to_string(), vec![
						CommandDefinition{
							regex: r"TST[ \t]+(?P<destination>[rR][0-7]),[ \t]+(?P<source>[rR][0-7])".to_string(),
							segments: vec![(SegType::Op,"8".to_string()),(SegType::Destination,"destination".to_string()),(SegType::Source,"source".to_string())],
							format_index: 4
						},
					]),
				]
			}
		}
	}
}

pub mod processor {
	pub enum SegType {
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
	/// A default def of a processor inspired by arm thumb mode.
	impl Default for ProcessorDefinition {
		fn default() -> Self {
			ProcessorDefinition {
				name: "ARM Thumbv1".to_string(),
				num_register: 16,
				register_size: 16,
				formats: vec![
					Format {
						id: 1,
						name: "move shifted register".to_string(),
						segments: vec![
							// Main: Mask: 1110 0000 0000 0000 Values: [0]: 000
							OperationSeg { name: None, mask: vec![0b11100000,0], seg_type: SegType::Main, values: Some(vec![vec![0,0]])},
							// OP: Mask: 0001 1000 0000 0000 Values: [LSL]: 00, [LSR]: 01, [ASR]: 10
							OperationSeg { name: Some("Op".to_string()), mask: vec![0b00011000,0], seg_type: SegType::Op, values: Some(vec![vec![0,0], vec![0b00001000,0], vec![0b00010000, 0]])},
							// Offset5: Mask: 0000 0111 1100 0000 Values: Any
							OperationSeg { name: Some("Offset5".to_string()), mask: vec![0b00000111,0b11000000], seg_type: SegType::Immediate, values: None},
							// Source register: Mask: 0000 0000 0011 1000 Values: Any
							OperationSeg { name: Some("Rs".to_string()), mask: vec![0,0b00111000], seg_type: SegType::Source, values: None},
							// Destination register: Mask: 0000 0000 0000 0111 Values: Any
							OperationSeg { name: Some("Rd".to_string()), mask: vec![0,0b00000111], seg_type: SegType::Destination, values: None},
						]
					},
					Format {
						id: 2,
						name: "add/subtract".to_string(),
						segments: vec![
							// Main: Mask: 1111 1000 0000 0000 Values: [0]: 0001 1
							OperationSeg { name: None, mask: vec![0b11111000,0], seg_type: SegType::Main, values: Some(vec![vec![0b00011000,0]])},
							// Immediate flag: Mask: 0000 0010 0000 0000 Values: all  [ADD]: 0, [SUB]: 1
							OperationSeg { name: Some("I".to_string()), mask: vec![0b00000100,0], seg_type: SegType::Flag, values: None},
							// OP: Mask: 0000 0010 0000 0000 Values: all [Register]: 0, [Immediate]: 1
							OperationSeg { name: Some("Op".to_string()), mask: vec![0b00000010,0], seg_type: SegType::Op, values: None},
							// Rn/Offset3: Mask: 0000 0001 1100 0000 Values: Any
							OperationSeg { name: Some("Rn/Offset3".to_string()), mask: vec![0b00000001,0b11000000], seg_type: SegType::Immediate, values: None},
							// Source register: Mask: 0000 0000 0011 1000 Values: Any
							OperationSeg { name: Some("Rs".to_string()), mask: vec![0,0b00111000], seg_type: SegType::Source, values: None},
							// Destination register: Mask: 0000 0000 0000 0111 Values: Any
							OperationSeg { name: Some("Rd".to_string()), mask: vec![0,0b00000111], seg_type: SegType::Destination, values: None},
						]
					},
					Format {
						id: 3,
						name: "move/compare/add/subtract immediate".to_string(),
						segments: vec![
							// Main: Mask: 1110 0000 0000 0000 Values: [0]: 001
							OperationSeg { name: None, mask: vec![0b11100000,0], seg_type: SegType::Main, values: Some(vec![vec![0b00100000,0]])},
							// OP: Mask: 0001 1000 0000 0000 Values: all [MOV]: 00, [CMP]: 01, [ADD]: 10, [SUB]: 11
							OperationSeg { name: Some("Op".to_string()), mask: vec![0b00011000,0], seg_type: SegType::Op, values: None},
							// Source/Destination register: Mask: 0000 0111 0000 0000 Values: Any
							OperationSeg { name: Some("Rd".to_string()), mask: vec![0b00000111,0], seg_type: SegType::Destination, values: None},
							// Offset8: Mask: 0000 0000 1111 1111 Values: Any
							OperationSeg { name: Some("Offset8".to_string()), mask: vec![0,0b11111111], seg_type: SegType::Immediate, values: None},
						]
					},
					Format {
						id: 4,
						name: "ALU operations".to_string(),
						segments: vec![
							// Main: Mask: 1111 1100 0000 0000 Values: [0]: 0100 00
							OperationSeg { name: None, mask: vec![0b11111100,0], seg_type: SegType::Main, values: Some(vec![vec![0b01000000,0]])},
							// OP: Mask: 0000 0011 1100 0000 Values: Any
							OperationSeg { name: Some("OP".to_string()), mask: vec![0b00000011,0b1100000], seg_type: SegType::Op, values: None},
							// Source register 2: 0000 0000 0011 1000 Values: Any
							OperationSeg { name: Some("Rs".to_string()), mask: vec![0,0b00111000], seg_type: SegType::Source, values: None},
							// Source/destination register: Mask: 0000 0000 0000 0111 Values: Any
							OperationSeg { name: Some("Rd".to_string()), mask: vec![0,0b00000111], seg_type: SegType::Destination, values: None},
						]
					},
					Format {
						id: 5,
						name: "hi register operations/branch exchange".to_string(),
						segments: vec![
							// Main: Mask: 1111 1100 0000 0000 Values: [0]: 0100 01
							OperationSeg { name: None, mask: vec![0b11111100,0], seg_type: SegType::Main, values: Some(vec![vec![0b01000100,0]])},
							// OP: Mask: 0000 0011 0000 0000 Values: all [ADD]: 00, [CMP]: 01, [MOV]: 10, [BX]: 11
							OperationSeg { name: Some("Op".to_string()), mask: vec![0b00000011,0], seg_type: SegType::Op, values: None},
							// Hi operand flag 1: Mask: 0000 0000 1000 0000 Values: any [LOW]: 0, [HI]: 1
							OperationSeg { name: Some("H1".to_string()), mask: vec![0,0b1000000], seg_type: SegType::Flag, values: None},
							// Hi operand flag 2: Mask: 0000 0000 0100 0000 Values: Any [LOW]: 0, [HI]: 1
							OperationSeg { name: Some("H2".to_string()), mask: vec![0,0b01000000], seg_type: SegType::Immediate, values: None},
							// Source register: Mask: 0000 0000 0011 1000 Values: Any
							OperationSeg { name: Some("Rs/Hs".to_string()), mask: vec![0,0b00111000], seg_type: SegType::Source, values: None},
							// Destination register: Mask: 0000 0000 0000 0111 Values: Any
							OperationSeg { name: Some("Rd/Hd".to_string()), mask: vec![0,0b00000111], seg_type: SegType::Destination, values: None},
						]
					},
					Format {
						id: 6,
						name: "PC-relative load".to_string(),
						segments: vec![
							// Main: Mask: 1111 1000 0000 0000 Values: [0]: 0100 1
							OperationSeg { name: None, mask: vec![0b11111000,0], seg_type: SegType::Main, values: Some(vec![vec![0b01001000,0]])},
							// Destination register: Mask: 0000 0111 0000 0000 Values: Any
							OperationSeg { name: Some("Rd".to_string()), mask: vec![0b00000111,0], seg_type: SegType::Op, values: None},
							// Immediate value: 0000 0000 1111 1111 Values: Any
							OperationSeg { name: Some("Word8".to_string()), mask: vec![0,0b11111111], seg_type: SegType::Source, values: None},
						]
					},
					Format {
						id: 7,
						name: "load/store with register offset".to_string(),
						segments: vec![
							// Main: Mask: 1111 0010 0000 0000 Values: [0]: 0101xx0 
							OperationSeg { name: None, mask: vec![0b11110010,0], seg_type: SegType::Main, values: Some(vec![vec![0b01010000,0]])},
							// Load/Store flag: Mask: 0000 1000 0000 0000 Values: all [Store to memory]: 0, [Load from memory]: 1
							OperationSeg { name: Some("L".to_string()), mask: vec![0b00001000,0], seg_type: SegType::Flag, values: None},
							// Byte/Word flag: Mask: 0000 0100 0000 0000 Values: any [Transfer word quantity]: 0, [Transfer byte quantity]: 1
							OperationSeg { name: Some("B".to_string()), mask: vec![0b00000100,0], seg_type: SegType::Flag, values: None},
							// Offset register: Mask: 0000 0001 1100 0000 Values: Any
							OperationSeg { name: Some("Ro".to_string()), mask: vec![0b00000001,0b11000000], seg_type: SegType::Offset, values: None},
							// Base register: Mask: 0000 0000 0011 1000 Values: Any
							OperationSeg { name: Some("Rb".to_string()), mask: vec![0,0b00111000], seg_type: SegType::Source, values: None},
							// Destination register: Mask: 0000 0000 0000 0111 Values: Any
							OperationSeg { name: Some("Rd".to_string()), mask: vec![0,0b00000111], seg_type: SegType::Destination, values: None},
						]
					},
					Format {
						id: 8,
						name: "load/store sign-extended byte/halfword".to_string(),
						segments: vec![
							// Main: Mask: 1111 0010 0000 0000 Values: [0]: 0101xx1 
							OperationSeg { name: None, mask: vec![0b11110010,0], seg_type: SegType::Main, values: Some(vec![vec![0b01010010,0]])},
							// H flag: Mask: 0000 1000 0000 0000 Values: all [Store to memory]: 0, [Load from memory]: 1 OR [Load sign byte]: 0, [load sign halfword]: 1
							OperationSeg { name: Some("H".to_string()), mask: vec![0b00001000,0], seg_type: SegType::Flag, values: None},
							// Sign-extended flag flag: Mask: 0000 0100 0000 0000 Values: any [Not sign extended]: 0, [Sign extended]: 1
							OperationSeg { name: Some("S".to_string()), mask: vec![0b00000100,0], seg_type: SegType::Flag, values: None},
							// Offset register: Mask: 0000 0001 1100 0000 Values: Any
							OperationSeg { name: Some("Ro".to_string()), mask: vec![0b00000001,0b11000000], seg_type: SegType::Offset, values: None},
							// Base register: Mask: 0000 0000 0011 1000 Values: Any
							OperationSeg { name: Some("Rb".to_string()), mask: vec![0,0b00111000], seg_type: SegType::Source, values: None},
							// Destination register: Mask: 0000 0000 0000 0111 Values: Any
							OperationSeg { name: Some("Rd".to_string()), mask: vec![0,0b00000111], seg_type: SegType::Destination, values: None},
						]
					},
					Format {
						id: 9,
						name: "load/store with immediate offset".to_string(),
						segments: vec![
							// Main: Mask: 1110 0000 0000 0000 Values: [0]: 011 
							OperationSeg { name: None, mask: vec![0b11100000,0], seg_type: SegType::Main, values: Some(vec![vec![0b01100000,0]])},
							// Byte/Word flag: Mask: 0001 0000 0000 0000 Values: all [Word]: 0, [Byte]: 1
							OperationSeg { name: Some("B".to_string()), mask: vec![0b00010000,0], seg_type: SegType::Flag, values: None},
							// Load/Store flag flag: Mask: 0000 1000 0000 0000 Values: any [Store]: 0, [Load]: 1
							OperationSeg { name: Some("L".to_string()), mask: vec![0b00001000,0], seg_type: SegType::Flag, values: None},
							// Offset Value: Mask: 0000 0111 1100 0000 Values: Any
							OperationSeg { name: Some("Offset5".to_string()), mask: vec![0b00000111,0b11000000], seg_type: SegType::Offset, values: None},
							// Base register: Mask: 0000 0000 0011 1000 Values: Any
							OperationSeg { name: Some("Rb".to_string()), mask: vec![0,0b00111000], seg_type: SegType::Source, values: None},
							// Destination register: Mask: 0000 0000 0000 0111 Values: Any
							OperationSeg { name: Some("Rd".to_string()), mask: vec![0,0b00000111], seg_type: SegType::Destination, values: None},
						]
					},
					Format {
						id: 10,
						name: "load/store halfword".to_string(),
						segments: vec![
							// Main: Mask: 1111 0000 0000 0000 Values: [0]: 1000 
							OperationSeg { name: None, mask: vec![0b11110000,0], seg_type: SegType::Main, values: Some(vec![vec![0b10000000,0]])},
							// Load/Store flag: Mask: 0000 1000 0000 0000 Values: all [Store]: 0, [Load]: 1
							OperationSeg { name: Some("L".to_string()), mask: vec![0b00000100,0], seg_type: SegType::Flag, values: None},
							// Immediate value: Mask: 0000 0111 1100 0000 Values: Any 
							OperationSeg { name: Some("Offset5".to_string()), mask: vec![0b00000111,0b11000000], seg_type: SegType::Immediate, values: None},
							// Base register: Mask: 0000 0000 0011 1000 Values: Any
							OperationSeg { name: Some("Rb".to_string()), mask: vec![0,0b00111000], seg_type: SegType::Source, values: None},
							// Destination register: Mask: 0000 0000 0000 0111 Values: Any
							OperationSeg { name: Some("Rd".to_string()), mask: vec![0,0b00000111], seg_type: SegType::Destination, values: None},
						]
					},
					Format {
						id: 11,
						name: "SP-relative load/store".to_string(),
						segments: vec![
							// Main: Mask: 1111 0000 0000 0000 Values: [0]: 1001 
							OperationSeg { name: None, mask: vec![0b11110000,0], seg_type: SegType::Main, values: Some(vec![vec![0b10010000,0]])},
							// Load/Store flag: Mask: 0000 1000 0000 0000 Values: all [Store]: 0, [Load]: 1
							OperationSeg { name: Some("L".to_string()), mask: vec![0b00000100,0], seg_type: SegType::Flag, values: None},
							// Destination register: Mask: 0000 0111 0000 0000 Values: Any 
							OperationSeg { name: Some("Rd".to_string()), mask: vec![0b00000111,0], seg_type: SegType::Destination, values: None},
							// Immediate value: Mask: 0000 0000 1111 1111 Values: Any
							OperationSeg { name: Some("Word8".to_string()), mask: vec![0,0b11111111], seg_type: SegType::Immediate, values: None},
						]
					},
					Format {
						id: 12,
						name: "load adddress".to_string(),
						segments: vec![
							// Main: Mask: 1111 0000 0000 0000 Values: [0]: 1010 
							OperationSeg { name: None, mask: vec![0b11110000,0], seg_type: SegType::Main, values: Some(vec![vec![0b10100000,0]])},
							// Source flag: Mask: 0000 1000 0000 0000 Values: all [Store]: 0, [Load]: 1
							OperationSeg { name: Some("SP".to_string()), mask: vec![0b00000100,0], seg_type: SegType::Flag, values: None},
							// Destination register: Mask: 0000 0111 0000 0000 Values: Any 
							OperationSeg { name: Some("Rd".to_string()), mask: vec![0b00000111,0], seg_type: SegType::Destination, values: None},
							// 8-bit unsigned constant: 0000 0000 1111 1111 Values: Any
							OperationSeg { name: Some("Word8".to_string()), mask: vec![0,0b11111111], seg_type: SegType::Immediate, values: None},
						]
					},
					Format {
						id: 13,
						name: "load adddress".to_string(),
						segments: vec![
							// Main: Mask: 1111 1111 0000 0000 Values: [0]: 10110000 
							OperationSeg { name: None, mask: vec![0b11111111,0], seg_type: SegType::Main, values: Some(vec![vec![0b10110000,0]])},
							// Sign flag: Mask: 0000 0000 1000 0000 Values: all [Positive]: 0, [Negative]: 1
							OperationSeg { name: Some("S".to_string()), mask: vec![0,0b10000000], seg_type: SegType::Flag, values: None},
							// 7-bit immediate value: 0000 0000 0111 1111 Values: Any
							OperationSeg { name: Some("SWord7".to_string()), mask: vec![0,0b01111111], seg_type: SegType::Immediate, values: None},
						]
					},
					Format {
						id: 14,
						name: "push/pop register".to_string(),
						segments: vec![
							// Main: Mask: 1111 0110 0000 0000 Values: [0]: 1011x10x 
							OperationSeg { name: None, mask: vec![0b11110110,0], seg_type: SegType::Main, values: Some(vec![vec![0b10110100,0]])},
							// Load/Store flag: Mask: 0000 1000 0000 0000 Values: all [Store]: 0, [Load]: 1
							OperationSeg { name: Some("L".to_string()), mask: vec![0b00001000,0], seg_type: SegType::Flag, values: None},
							// PC/LR flag: Mask: 0000 0001 0000 0000 Values: all [do not store]: 0, [store]: 1
							OperationSeg { name: Some("R".to_string()), mask: vec![0b00000001,0], seg_type: SegType::Flag, values: None},
							// register list value: 0000 0000 1111 1111 Values: Any
							OperationSeg { name: Some("Rlist".to_string()), mask: vec![0,0b11111111], seg_type: SegType::Immediate, values: None},
						]
					},
					Format {
						id: 15,
						name: "multiple load/store".to_string(),
						segments: vec![
							// Main: Mask: 1111 0000 0000 0000 Values: [0]: 1100 
							OperationSeg { name: None, mask: vec![0b11110000,0], seg_type: SegType::Main, values: Some(vec![vec![0b11000000,0]])},
							// Load/Store flag: Mask: 0000 1000 0000 0000 Values: all [Store]: 0, [Load]: 1
							OperationSeg { name: Some("L".to_string()), mask: vec![0b00001000,0], seg_type: SegType::Flag, values: None},
							// Base register: Mask: 0000 0111 0000 0000 Values: any
							OperationSeg { name: Some("Rb".to_string()), mask: vec![0b00000111,0], seg_type: SegType::Source, values: None},
							// register list value: 0000 0000 1111 1111 Values: Any
							OperationSeg { name: Some("Rlist".to_string()), mask: vec![0,0b11111111], seg_type: SegType::Immediate, values: None},
							]
					},
					Format {
						id: 16,
						name: "conditional branch".to_string(),
						segments: vec![
							// Main: Mask: 1111 0000 0000 0000 Values: [0]: 1101 
							OperationSeg { name: None, mask: vec![0b11110000,0], seg_type: SegType::Main, values: Some(vec![vec![0b11010000,0]])},
							// Condition: Mask: 0000 1111 0000 0000 Values: most, not 1110 or 1111 // ToDO fix values
							OperationSeg { name: Some("Cond".to_string()), mask: vec![0b00001111,0], seg_type: SegType::Condition, values: None},
							// 8-bit signed immediate: Mask: 0000 0000 1111 1111 Values: Any
							OperationSeg { name: Some("SOffset8".to_string()), mask: vec![0,0b11111111], seg_type: SegType::Immediate, values: None},
							]
					},
					Format {
						id: 17,
						name: "software interrupt".to_string(),
						segments: vec![
							// Main: Mask: 1111 1111 0000 0000 Values: [0]: 11011111 
							OperationSeg { name: None, mask: vec![0b11111111,0], seg_type: SegType::Main, values: Some(vec![vec![0b11011111,0]])},
							// Comment field: Mask: 0000 0000 1111 1111 Values: Any
							OperationSeg { name: Some("SOffset8".to_string()), mask: vec![0,0b11111111], seg_type: SegType::Immediate, values: None},
							]
					},
					Format {
						id: 18,
						name: "unconditional branch".to_string(),
						segments: vec![
							// Main: Mask: 1111 1000 0000 0000 Values: [0]: 1110 
							OperationSeg { name: None, mask: vec![0b11111000,0], seg_type: SegType::Main, values: Some(vec![vec![0b11100000,0]])},
							// Immediate value: Mask: 0000 0111 1111 1111 Values: Any
							OperationSeg { name: Some("Offset11".to_string()), mask: vec![0b00000111,0b11111111], seg_type: SegType::Immediate, values: None},
							]
					},
					Format {
						id: 19,
						name: "long branch with link".to_string(),
						segments: vec![
							// Main: Mask: 1111 0000 0000 0000 Values: [0]: 1111 
							OperationSeg { name: None, mask: vec![0b11110000,0], seg_type: SegType::Main, values: Some(vec![vec![0b11100000,0]])},
							// Low/High offset flag: Mask: 0000 1000 0000 0000 Values: all [high]: 0, [low]: 1
							OperationSeg { name: Some("H".to_string()), mask: vec![0b00001000,0], seg_type: SegType::Flag, values: None},
							// Long branch and link offset high/low: Mask: 0000 0111 1111 1111 Values: Any
							OperationSeg { name: Some("Offset".to_string()), mask: vec![0b00000111,0b11111111], seg_type: SegType::Immediate, values: None},
							]
					},
				]
			}
		}
	}
}

pub mod device {

}


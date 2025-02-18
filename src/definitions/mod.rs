


pub mod prelude {
	use super::language::*;
	use super::processor::*;
	use super::device::*;
}


pub mod language {

	use regex::Regex;
	use super::processor::ProcessorDefinition;
	pub struct CommandDefinition {
		pub command: String,
		pub operands: Option<Vec<String>>,
		pub command_bin: Vec<u8>,
		pub command_mask: Vec<u8>,
	}

	pub struct LanguageDefinition {
		pub comment_marker_regex: String,
		pub label_name_regex: String,
		pub label_marker: String,
		pub literal_regex: String,
		pub op_code_regex: String,
		pub max_operands: i32,
		pub operand_regex: String,
		pub operand_delim_regex: String,
		pub white_space_regex: String,
		pub compile_marker_regex: String,
		pub processor_def: ProcessorDefinition,
		pub commands: Vec<CommandDefinition>,
	}


	impl Default for LanguageDefinition {
		fn default() -> Self {
			LanguageDefinition {
				comment_marker_regex: r"@".to_string(),
				label_name_regex: r"[a-zA-Z_][a-zA-Z0-9_]*".to_string(),
				label_marker: r":".to_string(),
				literal_regex: r#"(#[0-9][0-9_x]?[0-9]*|"[\w\s]*")"#.to_string(),
				op_code_regex: r"[_a-zA-Z]+".to_string(),
				max_operands: 2,
				operand_regex: r"[a-zA-Z0-9_]+".to_string(),
				operand_delim_regex: r"[ ,]".to_string(),
				white_space_regex: r"[ \t]*".to_string(),
				compile_marker_regex: r"\.[a-zA-Z]+[\s]".to_string(),
				processor_def: Default::default(),
				commands: vec![]
			}
		}
	}

	impl LanguageDefinition {
		fn comment_regex_string(&self) -> String {
			let re_str = format!("(?P<comment>{}.*)?", self.comment_marker_regex);
			re_str
		}

		pub fn comment_regex(&self) -> Regex {
			let re = Regex::new(self.comment_regex_string().as_str()).unwrap();
			re
		}

		fn label_regex_string(&self) -> String {
			let re_str = format!("(?:(?P<label>{})[ \\t]*{})?", self.label_name_regex, self.label_marker);
			re_str
		}

		pub fn label_regex(&self) -> Regex {
			let re = Regex::new(self.label_regex_string().as_str()).unwrap();
			re
		}

		fn literal_regex_string(&self) -> String {
			let re_str = format!("(?:{ws}{d}?{ws}(?P<literal>{}))?", self.literal_regex, ws=self.white_space_regex, d=self.operand_delim_regex);
			re_str
		}

		pub fn literal_regex(&self) -> Regex {
			let re = Regex::new(&self.literal_regex_string()).unwrap();
			re
		}

		fn opc_regex_string(&self) -> String {
			let re_str = format!("(?P<opc>{})", self.op_code_regex);
			re_str
		}

		fn op_regex_string(&self, i: i32) -> String {
			let re_str = format!(r"(?P<op{}>{})", i.to_string(), self.operand_regex);
			re_str
		}

		fn compile_mark_regex_string(&self) -> String {
			let re_str = format!(r"(?P<compliemark>{})?", self.compile_marker_regex);
			re_str
		}
		/// Create a regex string for finding a command and its non literal operands
		fn command_regex_string(&self) -> String {
			// the nested set of operands, empty to start
			let mut re_ops = "".to_string();

			// nest operands staring for the last to the second
			for i in (1..self.max_operands).rev() {
				re_ops = format!(r"(?:{ws}{d}{ws}{op}{prev})?", ws=self.white_space_regex, d=self.operand_delim_regex, op=self.op_regex_string(i), prev=re_ops);
			}

			// nest the second - last operands in with the first
			re_ops = format!(r"(?:{ws}{first}{ws}{rest})?", first=self.op_regex_string(0), ws=self.white_space_regex, rest = re_ops);

			// combine with opcode regex to form full command string
			let re_str =  format!(r"(?:{opc}{ws}(?:{ops})?)?", opc=self.opc_regex_string(), ops=re_ops,ws = self.white_space_regex);
			re_str
		}

		pub fn line_regex_string(&self) -> String {
			let re_label = self.label_regex_string();
			let re_comment = self.comment_regex_string();
			let re_command =self.command_regex_string();
			let re_comp_mark = self.compile_mark_regex_string();
			let re_literal_mark = self.literal_regex_string();
			let re_str = format!(r"^{ws}{l}{ws}{cm}{ws}{cmd}{ws}{lit}{ws}{com}$", l=re_label, cm=re_comp_mark, cmd=re_command, lit=re_literal_mark, com=re_comment, ws = self.white_space_regex);
			re_str
		}

		/// returns a Regex for using the definition LanguageDefinition's values for a line of code
		pub fn line_regex(&self) -> Regex {
			let re = Regex::new(&self.line_regex_string()).unwrap();
			re 
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
							// Source register: Mask: 0000 0111 0000 0000 Values: Any
							OperationSeg { name: Some("Rd".to_string()), mask: vec![0b00000111,0], seg_type: SegType::Source, values: None},
							// Offset8: Mask: 0000 0000 1111 1111 Values: Any
							OperationSeg { name: Some("Offset8".to_string()), mask: vec![0,0b11111111], seg_type: SegType::Destination, values: None},
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


#[cfg(test)]
mod tests {

    use super::language::LanguageDefinition;

    #[test]
    fn create_line_regex() {
        let expected = r#"^[ \t]*(?:(?P<label>[a-zA-Z_][a-zA-Z0-9_]*)[ \t]*:)?[ \t]*(?P<compliemark>\.[a-zA-Z]+[\s])?[ \t]*(?:(?P<opc>[_a-zA-Z]+)[ \t]*(?:(?:[ \t]*(?P<op0>[a-zA-Z0-9_]+)[ \t]*(?:[ \t]*[ ,][ \t]*(?P<op1>[a-zA-Z0-9_]+))?)?)?)?[ \t]*(?:[ \t]*[ ,]?[ \t]*(?P<literal>(#[0-9][0-9_x]?[0-9]*|"[\w\s]*")))?[ \t]*(?P<comment>@.*)?$"#;
		let default_lang: LanguageDefinition = Default::default();

		let got = default_lang.line_regex_string();

		assert_eq!(got, expected);

		println!("{}", got);

    }
}

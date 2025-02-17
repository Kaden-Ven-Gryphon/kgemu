


pub mod prelude {
	use super::language::*;
	use super::processor::*;
	use super::device::*;
}


pub mod language {

	use regex::Regex;
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
		pub literal_num_regex: String,
		pub command_regex: String,
		pub max_operands: i32,
		pub operand_regex: String,
		pub operand_delim_regex: String,
		pub white_space_regex: String,
		pub commands: Vec<CommandDefinition>,
	}

//"^[ \t]*((?P<label>[a-zA-Z_][a-zA-Z0-9_]*)[ \t]*:)?[ \t]*((?P<opc>[a-zA-Z]+)[ \t]*(([ \t](?P<op1>[a-zA-Z0-9_]+)([ \t]*,[ \t]*(?P<op2>[a-zA-Z0-9_]+))?)?([ \t]*,?[ \t](?P<offset>#[0-9][0-9_x]?[0-9]*))?)?)?[ \t]*(?P<comment>@.*)?$"gm

	impl Default for LanguageDefinition {
		fn default() -> Self {
			LanguageDefinition {
				comment_marker_regex: r"@".to_string(),
				label_name_regex: r"[a-zA-Z_][a-zA-Z0-9_]*".to_string(),
				label_marker: r":".to_string(),
				literal_num_regex: r"#[0-9][0-9_x]?[0-9]*".to_string(),
				command_regex: r"[a-zA-Z]+".to_string(),
				max_operands: 2,
				operand_regex: r"[a-zA-Z0-9_]+".to_string(),
				operand_delim_regex: r"[ ,]".to_string(),
				white_space_regex: r"[ \t]*".to_string(),
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

		fn offset_regex_string(&self) -> String {
			let re_str = format!("(?P<offset>{})", self.literal_num_regex);
			re_str
		}

		pub fn offset_regex(&self) -> Regex {
			let re = Regex::new(&self.offset_regex_string()).unwrap();
			re
		}

		fn opc_regex_string(&self) -> String {
			let re_str = format!("(?P<opc>{})", self.command_regex);
			re_str
		}

		fn op_regex_string(&self, i: i32) -> String {
			let re_str = format!(r"(?P<op{}>{})", i.to_string(), self.operand_regex);
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
			let re_str =  format!(r"(?:{}{ws}(?:{}(?:{ws}{}?{ws}{})?)?)?", self.opc_regex_string(), re_ops, self.operand_delim_regex, self.offset_regex_string(), ws = self.white_space_regex);
			re_str
		}

		pub fn line_regex_string(&self) -> String {
			let re_label = self.label_regex_string();
			let re_comment = self.comment_regex_string();
			let re_command =self.command_regex_string();
			let re_str = format!(r"^{ws}{}{ws}{}{ws}{}$", re_label,re_command, re_comment, ws = self.white_space_regex);
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
	enum SegType {
		Main,
		Op,
		Flag,
		Source,
		Destination,
		Immediate
	}
	pub struct OperationSeg {
		pub mask: Vec<u8>,
		pub seg_type: SegType,
	}
	pub struct Format {
		pub segments: Vec<OperationSeg>,
	}
	
	pub struct ProcessorDefinition {
		pub name: String,
		pub num_register: i32,
		pub register_size: i32,
		
	}
}

pub mod device {

}


#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::{language::LanguageDefinition, *};

    #[test]
    fn create_line_regex() {
        let expected = r"^[ \t]*((?P<label>[a-zA-Z_][a-zA-Z0-9_]*)[ \t]*:)?[ \t]*((?P<opc>[a-zA-Z]+)[ \t]*(([ \t]*(?P<op0>[a-zA-Z0-9_]+)[ \t]*([ \t]*[ ,][ \t]*(?P<op1>[a-zA-Z0-9_]+))?)?([ \t]*[ ,]?[ \t]*(?P<offset>#[0-9][0-9_x]?[0-9]*))?)?)?[ \t]*(?P<comment>@.*)?$";
		let default_lang: LanguageDefinition = Default::default();

		let got = default_lang.line_regex_string();

		assert_eq!(got, expected);

    }
}

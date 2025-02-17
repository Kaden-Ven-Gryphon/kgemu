

pub struct VirtualProcessor {
	pub name: String,
	pub clock_speed: i32,
	//pub language: LanguageDefinition,
	pub num_registers: i32,
	pub registers_size: i32,
	registers: Vec<Vec<u8>>,
	pub ram_size: i32,
	ram_data: Vec<u8>,
	pub rom_size: i32,
	rom_data: Vec<u8>,

}

impl VirtualProcessor {
	pub fn set_rom(&mut self, data: Vec<u8>) {
		self.rom_data = data;
	}
}
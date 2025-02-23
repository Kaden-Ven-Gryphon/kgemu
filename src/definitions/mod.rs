// definitions modual
// has the structs for defiinitions of processors, languages, and hardware

pub mod processor;
pub mod language;
pub mod device;

pub mod thumb_default;	



pub mod prelude {
	pub use super::language::prelude::*;
	pub use super::processor::prelude::*;
	pub use super::device::prelude::*;
}

/// kgemu is a crate for compileing and emulating assembly code


pub mod compile;
pub mod emulate;
pub mod definitions;
pub mod virtual_processor;

pub mod prelude {
    pub use crate::compile::prelude::*;
    pub use crate::emulate::prelude::*;
    pub use crate::definitions::prelude::*;
}





pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

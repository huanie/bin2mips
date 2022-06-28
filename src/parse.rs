use std::fmt::Display;

use crate::types::{I_Format, J_Format, R_Format};

#[allow(non_camel_case_types)]
#[allow(clippy::enum_variant_names)]
pub enum Format {
    I_Format(u32),
    R_Format(u32),
    J_Format(u32),
}

impl Format {
    pub fn new(input: &str) -> Format {
        let bin: u32 = u32::from_str_radix(input, 2).unwrap();

        if (bin >> 26) > 63 {
            panic!("This is not an opcode");
        }
        match bin >> 26 {
            0b000000 => Format::R_Format(bin),
            0b000010 | 0b000011 => Format::J_Format(bin),
            _ => Format::I_Format(bin),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::I_Format(data) => write!(f, "{}", I_Format::new(*data)),
            Format::R_Format(data) => write!(f, "{}", R_Format::new(*data)),
            Format::J_Format(data) => write!(f, "{}", J_Format::new(*data)),
        }
    }
}

use std::fmt::Display;

use crate::types::{I_Format, J_Format, MappedCSV, R_Format};

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

        if let 0b000000 = bin >> 26 {
            Format::R_Format(bin)
        } else {
            match MappedCSV::get_mapped_value(MappedCSV {
                filename: "commands.csv",
                check_column: "OP_BINARY",
                query: format!("{:06b}", bin >> 26),
                want_column: "FORMAT",
            })
            .as_str()
            {
                "I-Format" => Format::I_Format(bin),
                "J-Format" => Format::J_Format(bin),
                _ => panic!("Error at parsing!"),
            }
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

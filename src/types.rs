use std::fmt::Display;

#[allow(non_camel_case_types)]
pub struct I_Format {
    opcode: u32,
    rs_register: Register,
    rt_register: Register,
    immediate_constant: u32,
}

impl I_Format {
    pub fn new(bin: u32) -> I_Format {
        I_Format {
            opcode: bin >> 26,
            rs_register: Register::new((bin << 6) >> 27),
            rt_register: Register::new((bin << 11) >> 27),
            immediate_constant: (bin << 16) >> 16,
        }
    }

    fn find_operation(&self) -> String {
        csv_get_mapped_value(MappedCSV::command_search("OP_BINARY", self.opcode))
    }
}

impl Display for I_Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "(I-Format) {} {}, {}, {}\nrt: {:05b} is {}\nrs: {:05b} is {}",
            self.find_operation(),
            self.rt_register,
            self.rs_register,
            self.immediate_constant as i16,
            self.rt_register.code,
            self.rt_register,
            self.rs_register.code,
            self.rs_register
        )
    }
}

#[allow(non_camel_case_types)]
pub struct R_Format {
    rs_register: Register,
    rt_register: Register,
    rd_register: Register,
    shift_amount: u32,
    function_code: u32,
}

impl R_Format {
    pub fn new(bin: u32) -> R_Format {
        R_Format {
            rs_register: Register::new((bin << 6) >> 27),
            rt_register: Register::new((bin << 11) >> 27),
            rd_register: Register::new((bin << 16) >> 27),
            shift_amount: (bin << 21) >> 27,
            function_code: (bin << 26) >> 26,
        }
    }
    fn find_operation(&self) -> String {
        csv_get_mapped_value(MappedCSV::command_search("FUNC_BINARY", self.function_code))
    }
}

impl Display for R_Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.shift_amount != 0 {
            write!(
                f,
                "(R-Format) {} {}, {}, {}\nrt: {} is {}\nrd: {} is {}",
                self.find_operation(),
                self.rd_register,
                self.rt_register,
                self.shift_amount,
                self.rt_register.code,
                self.rt_register,
                self.rd_register.code,
                self.rd_register
            )
        } else {
            write!(
                f,
                "(R-Format) {} {}, {}, {}\nrs: {:05b} is {}\nrt: {:05b} is {}\nrd: {:05b} is {}",
                self.find_operation(),
                self.rd_register,
                self.rs_register,
                self.rt_register,
                self.rs_register.code,
                self.rs_register,
                self.rt_register.code,
                self.rt_register,
                self.rd_register.code,
                self.rd_register,
            )
        }
    }
}

#[allow(non_camel_case_types)]
pub struct J_Format {
    opcode: u32,
    address: u32,
}

impl J_Format {
    pub fn new(bin: u32) -> J_Format {
        J_Format {
            opcode: bin >> 26,
            address: (bin << 6) >> 6,
        }
    }

    fn find_operation(&self) -> String {
        csv_get_mapped_value(MappedCSV::command_search("OP_BINARY", self.opcode))
    }
}

impl Display for J_Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(J-Format) {} {}\nJump to address {}",
            self.find_operation(),
            self.address,
            self.address * 4
        )
    }
}

pub struct Register {
    code: u32,
}

impl Register {
    fn new(bin: u32) -> Register {
        Register { code: bin }
    }

    fn register_name(&self) -> String {
        csv_get_mapped_value(MappedCSV::register_search(self.code))
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.register_name())
    }
}

struct MappedCSV<'a> {
    filename: &'a str,
    check_column: &'a str,
    query: String,
    want_column: &'a str,
}

impl MappedCSV<'_> {
    fn register_search(query: u32) -> MappedCSV<'static> {
        MappedCSV {
            filename: "registers.csv",
            check_column: "BINARY",
            query: format!("{:05b}", query),
            want_column: "REGISTER",
        }
    }

    fn command_search<'a>(check_column: &'a str, query: u32) -> MappedCSV<'a> {
        MappedCSV {
            filename: "commands.csv",
            check_column,
            query: format!("{:06b}", query),
            want_column: "COMMAND",
        }
    }
}

fn csv_get_mapped_value(csv_map: MappedCSV) -> String {
    let mut csv_reader = csv::Reader::from_path(csv_map.filename)
        .unwrap_or_else(|_| panic!("{} is not in the directory!", csv_map.filename));
    let header_iter = csv_reader.headers().unwrap().iter();

    let check_position = header_iter
        .clone()
        .position(|x| x == csv_map.check_column)
        .unwrap_or_else(|| panic!("{} column was not found.", csv_map.check_column));

    let want_position = header_iter
        .clone()
        .position(|x| x == csv_map.want_column)
        .unwrap_or_else(|| panic!("{} column was not found.", csv_map.want_column));

    csv_reader
        .records()
        .find(|row| row.as_ref().unwrap().get(check_position).unwrap() == csv_map.query)
        .unwrap_or_else(|| panic!("{} entry was not found.", csv_map.query))
        .unwrap()
        .get(want_position)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_register_name() {
        let result = Register::new(0b00100);

        assert_eq!(result.register_name(), "a0");
    }

    #[test]
    fn find_jump_command() {
        assert_eq!(
            "j 256\nJump to address 1024",
            J_Format::new(0b0000_1000_0000_0000_0000_0001_0000_0000).to_string()
        );
    }
}

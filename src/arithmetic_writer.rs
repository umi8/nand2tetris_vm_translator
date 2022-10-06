use std::fmt::{Error, Write};

use crate::arithmetic_type::ArithmeticType;

pub struct ArithmeticWriter {
    comparison_counter: i32,
}

impl ArithmeticWriter {
    pub fn new() -> Self {
        ArithmeticWriter { comparison_counter: 0 }
    }

    pub fn write(&self, command: &str) -> Result<String, Error> {
        let mut s = String::new();
        match ArithmeticType::from(command).unwrap() {
            ArithmeticType::ADD => self.add(&mut s),
            // ArithmeticType::SUB => self.sub(),
            // ArithmeticType::NEG => self.neg(),
            // ArithmeticType::EQ => self.eq(),
            // ArithmeticType::GT => self.gt(),
            // ArithmeticType::LT => self.lt(),
            // ArithmeticType::AND => self.and(),
            // ArithmeticType::OR => self.or(),
            // ArithmeticType::NOT => self.not()
            _ => self.sub()
        }?;
        self.add(&mut s)?;
        writeln!(s, "{}", command)?;
        writeln!(s, "{}", command)?;
        Ok(s)
    }

    fn add(&self, s: &mut String) -> Result<(), Error> {
        writeln!(s, "{}", "add_fn")?;
        Ok(())
    }
    fn sub(&self) -> Result<(), Error> {
        Ok(())
    }
}

use std::fmt::{Error, Write};

use crate::arithmetic_type::ArithmeticType;

pub fn write(command: &str) -> Result<String, Error> {
    let mut s = String::new();
    match ArithmeticType::from(command).unwrap() {
        ArithmeticType::ADD => add(&mut s),
        // ArithmeticType::SUB => self.sub(),
        // ArithmeticType::NEG => self.neg(),
        // ArithmeticType::EQ => self.eq(),
        // ArithmeticType::GT => self.gt(),
        // ArithmeticType::LT => self.lt(),
        // ArithmeticType::AND => self.and(),
        // ArithmeticType::OR => self.or(),
        // ArithmeticType::NOT => self.not()
        _ => sub()
    }?;
    add(&mut s)?;
    writeln!(s, "{}", command)?;
    writeln!(s, "{}", command)?;
    Ok(s)
}

fn add(s: &mut String) -> Result<(), Error> {
    writeln!(s, "{}", "add_fn")?;
    Ok(())
}

fn sub() -> Result<(), Error> {
    Ok(())
}

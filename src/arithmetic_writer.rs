use std::fmt::{Error, Write};

use crate::arithmetic_type::ArithmeticType;

pub fn write(command: &str, comparison_counter: i32) -> Result<(String, i32), Error> {
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
    writeln!(s, "{}", comparison_counter)?;
    let inc = match ArithmeticType::from(command).unwrap() {
        ArithmeticType::EQ | ArithmeticType::GT | ArithmeticType::LT => comparison_counter + 1,
        _ => comparison_counter
    };
    Ok((s, inc))
}

fn add(s: &mut String) -> Result<(), Error> {
    writeln!(s, "{}", "add_fn")?;
    Ok(())
}

fn sub() -> Result<(), Error> {
    Ok(())
}

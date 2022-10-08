use std::fmt::{Error, Write};

use crate::arithmetic_type::ArithmeticType;

pub fn write(command: &str, comparison_counter: i32) -> Result<(String, i32), Error> {
    let mut s = String::new();
    match ArithmeticType::from(command).unwrap() {
        ArithmeticType::ADD => add(&mut s),
        // ArithmeticType::SUB => sub(),
        // ArithmeticType::NEG => neg(),
        // ArithmeticType::EQ => eq(),
        // ArithmeticType::GT => gt(),
        // ArithmeticType::LT => lt(),
        // ArithmeticType::AND => and(),
        // ArithmeticType::OR => or(),
        // ArithmeticType::NOT => not()
        _ => sub()
    }?;
    let inc = match ArithmeticType::from(command).unwrap() {
        ArithmeticType::EQ | ArithmeticType::GT | ArithmeticType::LT => comparison_counter + 1,
        _ => comparison_counter
    };
    Ok((s, inc))
}

fn add(s: &mut String) -> Result<(), Error> {
    binary_operation(s, "+");
    Ok(())
}

fn sub() -> Result<(), Error> {
    Ok(())
}

fn binary_operation(s: &mut String, operator: &str) -> Result<(), Error> {
    peek_value_into_d_register(s)?;
    decrement_stack_pointer(s)?;
    set_memory_address_to_stack_pointer(s)?;
    writeln!(s, "M=M{}D", operator)?;
    increment_stack_pointer(s)?;
    Ok(())
}

fn peek_value_into_d_register(s: &mut String) -> Result<(), Error> {
    decrement_stack_pointer(s)?;
    set_memory_address_to_stack_pointer(s)?;
    // store top of stack value in D register
    writeln!(s, "D=M")?;
    Ok(())
}

fn set_memory_address_to_stack_pointer(s: &mut String) -> Result<(), Error> {
    writeln!(s, "@SP")?;
    writeln!(s, "A=M")?;
    Ok(())
}

fn increment_stack_pointer(s: &mut String) -> Result<(), Error> {
    writeln!(s, "@SP")?;
    writeln!(s, "M=M+1")?;
    Ok(())
}

fn decrement_stack_pointer(s: &mut String) -> Result<(), Error> {
    writeln!(s, "@SP")?;
    writeln!(s, "M=M-1")?;
    Ok(())
}

use std::fmt::{Error, Write};

use crate::arithmetic_type::ArithmeticType;

pub fn write(command: &ArithmeticType, comparison_counter: i32) -> Result<String, Error> {
    let mut s = String::new();
    match command {
        ArithmeticType::Add => add(&mut s),
        ArithmeticType::Sub => sub(&mut s),
        ArithmeticType::Neg => neg(&mut s),
        ArithmeticType::EQ => eq(&mut s, comparison_counter),
        ArithmeticType::GT => gt(&mut s, comparison_counter),
        ArithmeticType::LT => lt(&mut s, comparison_counter),
        ArithmeticType::And => and(&mut s),
        ArithmeticType::OR => or(&mut s),
        ArithmeticType::Not => not(&mut s),
    }?;
    Ok(s)
}

fn add(s: &mut String) -> Result<(), Error> {
    binary_operation(s, "+")?;
    Ok(())
}

fn sub(s: &mut String) -> Result<(), Error> {
    binary_operation(s, "-")?;
    Ok(())
}

fn neg(s: &mut String) -> Result<(), Error> {
    unary_operation(s, "-")?;
    Ok(())
}

fn eq(s: &mut String, comparison_counter: i32) -> Result<(), Error> {
    comparison(s, "JEQ", comparison_counter)?;
    Ok(())
}

fn gt(s: &mut String, comparison_counter: i32) -> Result<(), Error> {
    comparison(s, "JGT", comparison_counter)?;
    Ok(())
}

fn lt(s: &mut String, comparison_counter: i32) -> Result<(), Error> {
    comparison(s, "JLT", comparison_counter)?;
    Ok(())
}

fn and(s: &mut String) -> Result<(), Error> {
    binary_operation(s, "&")?;
    Ok(())
}

fn or(s: &mut String) -> Result<(), Error> {
    binary_operation(s, "|")?;
    Ok(())
}

fn not(s: &mut String) -> Result<(), Error> {
    unary_operation(s, "!")?;
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

fn unary_operation(s: &mut String, operator: &str) -> Result<(), Error> {
    decrement_stack_pointer(s)?;
    set_memory_address_to_stack_pointer(s)?;
    writeln!(s, "M={}M", operator)?;
    increment_stack_pointer(s)?;
    Ok(())
}

fn comparison(s: &mut String, jump_mnemonic: &str, comparison_counter: i32) -> Result<(), Error> {
    peek_value_into_d_register(s)?;
    decrement_stack_pointer(s)?;
    set_memory_address_to_stack_pointer(s)?;
    // D=x-y
    writeln!(s, "D=M-D")?;
    // set the destination address if true in A register
    writeln!(s, "@COMP{}", comparison_counter)?;
    // jump operation
    writeln!(s, "D;{}", jump_mnemonic)?;
    // set false
    set_memory_address_to_stack_pointer(s)?;
    writeln!(s, "M=0")?;
    // set the destination address of finally process in A register and jump
    writeln!(s, "@ENDCOMP{}", comparison_counter)?;
    writeln!(s, "0;JMP")?;
    // define a label if true
    writeln!(s, "(COMP{})", comparison_counter)?;
    // set true
    set_memory_address_to_stack_pointer(s)?;
    writeln!(s, "M=-1")?;
    // define a label for finally process
    writeln!(s, "(ENDCOMP{})", comparison_counter)?;
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

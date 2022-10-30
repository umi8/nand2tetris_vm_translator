use std::fmt::Write;

use anyhow::Result;

use crate::ArithmeticType;

pub fn write(arithmetic_type: &ArithmeticType, label_number: i32) -> Result<String> {
    let mut s = String::new();
    match arithmetic_type {
        ArithmeticType::Add => add(&mut s),
        ArithmeticType::Sub => sub(&mut s),
        ArithmeticType::Neg => neg(&mut s),
        ArithmeticType::Eq => eq(&mut s, label_number),
        ArithmeticType::Gt => gt(&mut s, label_number),
        ArithmeticType::Lt => lt(&mut s, label_number),
        ArithmeticType::And => and(&mut s),
        ArithmeticType::Or => or(&mut s),
        ArithmeticType::Not => not(&mut s),
    }?;
    Ok(s)
}

fn add(s: &mut String) -> Result<()> {
    binary_operation(s, "+")?;
    Ok(())
}

fn sub(s: &mut String) -> Result<()> {
    binary_operation(s, "-")?;
    Ok(())
}

fn neg(s: &mut String) -> Result<()> {
    unary_operation(s, "-")?;
    Ok(())
}

fn eq(s: &mut String, label_number: i32) -> Result<()> {
    comparison(s, "JEQ", label_number)?;
    Ok(())
}

fn gt(s: &mut String, label_number: i32) -> Result<()> {
    comparison(s, "JGT", label_number)?;
    Ok(())
}

fn lt(s: &mut String, label_number: i32) -> Result<()> {
    comparison(s, "JLT", label_number)?;
    Ok(())
}

fn and(s: &mut String) -> Result<()> {
    binary_operation(s, "&")?;
    Ok(())
}

fn or(s: &mut String) -> Result<()> {
    binary_operation(s, "|")?;
    Ok(())
}

fn not(s: &mut String) -> Result<()> {
    unary_operation(s, "!")?;
    Ok(())
}

fn binary_operation(s: &mut String, operator: &str) -> Result<()> {
    peek_value_into_d_register(s)?;
    decrement_stack_pointer(s)?;
    set_memory_address_to_stack_pointer(s)?;
    writeln!(s, "M=M{}D", operator)?;
    increment_stack_pointer(s)?;
    Ok(())
}

fn unary_operation(s: &mut String, operator: &str) -> Result<()> {
    decrement_stack_pointer(s)?;
    set_memory_address_to_stack_pointer(s)?;
    writeln!(s, "M={}M", operator)?;
    increment_stack_pointer(s)?;
    Ok(())
}

fn comparison(s: &mut String, jump_mnemonic: &str, label_number: i32) -> Result<()> {
    peek_value_into_d_register(s)?;
    decrement_stack_pointer(s)?;
    set_memory_address_to_stack_pointer(s)?;
    // D=x-y
    writeln!(s, "D=M-D")?;
    // set the destination address if true in A register
    writeln!(s, "@COMP{}", label_number)?;
    // jump operation
    writeln!(s, "D;{}", jump_mnemonic)?;
    // set false
    set_memory_address_to_stack_pointer(s)?;
    writeln!(s, "M=0")?;
    // set the destination address of finally process in A register and jump
    writeln!(s, "@ENDCOMP{}", label_number)?;
    writeln!(s, "0;JMP")?;
    // define a label if true
    writeln!(s, "(COMP{})", label_number)?;
    // set true
    set_memory_address_to_stack_pointer(s)?;
    writeln!(s, "M=-1")?;
    // define a label for finally process
    writeln!(s, "(ENDCOMP{})", label_number)?;
    increment_stack_pointer(s)?;
    Ok(())
}

fn peek_value_into_d_register(s: &mut String) -> Result<()> {
    decrement_stack_pointer(s)?;
    set_memory_address_to_stack_pointer(s)?;
    // store top of stack value in D register
    writeln!(s, "D=M")?;
    Ok(())
}

fn set_memory_address_to_stack_pointer(s: &mut String) -> Result<()> {
    writeln!(s, "@SP")?;
    writeln!(s, "A=M")?;
    Ok(())
}

fn increment_stack_pointer(s: &mut String) -> Result<()> {
    writeln!(s, "@SP")?;
    writeln!(s, "M=M+1")?;
    Ok(())
}

fn decrement_stack_pointer(s: &mut String) -> Result<()> {
    writeln!(s, "@SP")?;
    writeln!(s, "M=M-1")?;
    Ok(())
}

use std::fmt::{Error, Write};

use crate::CommandType;

pub fn write(command: CommandType, segment: &str, index: &i32) -> Result<String, Error> {
    let mut s = String::new();
    if command == CommandType::PUSH {
        push(&mut s, segment, index)?;
    } else if command == CommandType::POP {
        pop(&mut s, segment, index)?;
    }
    Ok(s)
}

fn push(s: &mut String, segment: &str, index: &i32) -> Result<(), Error> {
    if segment.eq("constant") {
        writeln!(s, "@{}", index)?;
        writeln!(s, "D=A")?;
        set_memory_address_to_stack_pointer(s)?;
        writeln!(s, "M=D")?;
        increment_stack_pointer(s)?;
    } else if segment.eq("pointer") {
        if *index == 0 {
            writeln!(s, "@THIS")?;
        } else if *index == 1 {
            writeln!(s, "@THAT")?;
        }
        writeln!(s, "D=M")?;
        set_memory_address_to_stack_pointer(s)?;
        writeln!(s, "M=D")?;
        increment_stack_pointer(s)?;
    } else {
        store_address_into_d_register(s, segment, index)?;
        writeln!(s, "A=D")?;
        writeln!(s, "D=M")?;
        set_memory_address_to_stack_pointer(s)?;
        writeln!(s, "M=D")?;
        increment_stack_pointer(s)?;
    }
    Ok(())
}

fn pop(s: &mut String, segment: &str, index: &i32) -> Result<(), Error> {
    if segment.eq("pointer") {
        if *index == 0 {
            writeln!(s, "@THIS")?;
        } else if *index == 1 {
            writeln!(s, "@THAT")?;
        }
        writeln!(s, "D=A")?;
        writeln!(s, "@R13")?;
        writeln!(s, "M=D")?;

        peek_value_into_d_register(s)?;

        writeln!(s, "@R13")?;
        writeln!(s, "A=M")?;

        writeln!(s, "M=D")?;
    } else {
        store_address_into_d_register(s, segment, index)?;

        writeln!(s, "@R13")?;
        writeln!(s, "M=D")?;

        peek_value_into_d_register(s)?;

        writeln!(s, "@R13")?;
        writeln!(s, "A=M")?;

        writeln!(s, "M=D")?;
    }
    Ok(())
}

fn store_address_into_d_register(s: &mut String, segment: &str, index: &i32) -> Result<(), Error> {
    if segment.eq("local") {
        writeln!(s, "@LCL")?;
        writeln!(s, "D=M")?;
    } else if segment.eq("argument") {
        writeln!(s, "@ARG")?;
        writeln!(s, "D=M")?;
    } else if segment.eq("this") {
        writeln!(s, "@THIS")?;
        writeln!(s, "D=M")?;
    } else if segment.eq("that") {
        writeln!(s, "@THAT")?;
        writeln!(s, "D=M")?;
    } else if segment.eq("temp") {
        writeln!(s, "@5")?;
        writeln!(s, "D=A")?;
    }

    writeln!(s, "@{}", index)?;
    writeln!(s, "D=D+A")?;
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

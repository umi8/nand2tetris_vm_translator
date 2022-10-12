use std::fmt::{Error, Write};

use crate::CommandType;
use crate::segment::Segment;

pub fn write(command: CommandType, segment: Segment, index: &i32) -> Result<String, Error> {
    let mut s = String::new();
    if command == CommandType::PUSH {
        push(&mut s, segment, index)?;
    } else if command == CommandType::POP {
        pop(&mut s, segment, index)?;
    }
    Ok(s)
}

fn push(s: &mut String, segment: Segment, index: &i32) -> Result<(), Error> {
    // segment[index]をDレジスタに入れる
    store_index_of_segment_in_d_register(s, segment, index)?;
    // SPをAレジスタに入れる
    set_memory_address_to_stack_pointer(s)?;
    // pushする
    writeln!(s, "M=D")?;
    // SPをインクリメントする
    increment_stack_pointer(s)?;
    Ok(())
}

fn pop(s: &mut String, segment: Segment, index: &i32) -> Result<(), Error> {
    // pop先のアドレスをDレジスタに入れる
    store_dest_address_in_d_register(s, segment, index)?;
    // pop先のアドレスをR13に退避する
    writeln!(s, "@R13")?;
    writeln!(s, "M=D")?;
    // popする値をDレジスタに入れる
    peek_value_into_d_register(s)?;
    // pop先のアドレスをR13から取り出してAレジスタに入れる
    writeln!(s, "@R13")?;
    writeln!(s, "A=M")?;
    // popする
    writeln!(s, "M=D")?;
    Ok(())
}

fn store_index_of_segment_in_d_register(s: &mut String, segment: Segment, index: &i32) -> Result<(), Error> {
    match segment {
        Segment::CONSTANT => {
            writeln!(s, "@{}", index)?;
            writeln!(s, "D=A")?;
        },
        Segment::LOCAL => {
            writeln!(s, "@LCL")?;
            writeln!(s, "D=M")?;
            writeln!(s, "@{}", index)?;
            writeln!(s, "D=D+A")?;
            writeln!(s, "A=D")?;
            writeln!(s, "D=M")?;
        },
        Segment::ARGUMENT => {
            writeln!(s, "@ARG")?;
            writeln!(s, "D=M")?;
            writeln!(s, "@{}", index)?;
            writeln!(s, "D=D+A")?;
            writeln!(s, "A=D")?;
            writeln!(s, "D=M")?;
        },
        Segment::THIS => {
            writeln!(s, "@THIS")?;
            writeln!(s, "D=M")?;
            writeln!(s, "@{}", index)?;
            writeln!(s, "D=D+A")?;
            writeln!(s, "A=D")?;
            writeln!(s, "D=M")?;
        },
        Segment::THAT => {
            writeln!(s, "@THAT")?;
            writeln!(s, "D=M")?;
            writeln!(s, "@{}", index)?;
            writeln!(s, "D=D+A")?;
            writeln!(s, "A=D")?;
            writeln!(s, "D=M")?;
        },
        Segment::TEMP => {
            writeln!(s, "@5")?;
            writeln!(s, "D=A")?;
            writeln!(s, "@{}", index)?;
            writeln!(s, "D=D+A")?;
            writeln!(s, "A=D")?;
            writeln!(s, "D=M")?;
        },
        Segment::POINTER => {
            if *index == 0 {
                writeln!(s, "@THIS")?;
            } else if *index == 1 {
                writeln!(s, "@THAT")?;
            }
            writeln!(s, "D=M")?;
        },
        Segment::STATIC => {
            writeln!(s, "@16")?;
            writeln!(s, "D=A")?;
            writeln!(s, "@{}", index)?;
            writeln!(s, "D=D+A")?;
            writeln!(s, "A=D")?;
            writeln!(s, "D=M")?;
        }
    }
    Ok(())
}

fn store_dest_address_in_d_register(s: &mut String, segment: Segment, index: &i32) -> Result<(), Error> {
    match segment {
        Segment::CONSTANT => {} // do nothing
        Segment::LOCAL => {
            writeln!(s, "@LCL")?;
            writeln!(s, "D=M")?;
            writeln!(s, "@{}", index)?;
            writeln!(s, "D=D+A")?;
        },
        Segment::ARGUMENT => {
            writeln!(s, "@ARG")?;
            writeln!(s, "D=M")?;
            writeln!(s, "@{}", index)?;
            writeln!(s, "D=D+A")?;
        },
        Segment::THIS => {
            writeln!(s, "@THIS")?;
            writeln!(s, "D=M")?;
            writeln!(s, "@{}", index)?;
            writeln!(s, "D=D+A")?;
        },
        Segment::THAT => {
            writeln!(s, "@THAT")?;
            writeln!(s, "D=M")?;
            writeln!(s, "@{}", index)?;
            writeln!(s, "D=D+A")?;
        },
        Segment::TEMP => {
            writeln!(s, "@5")?;
            writeln!(s, "D=A")?;
            writeln!(s, "@{}", index)?;
            writeln!(s, "D=D+A")?;
        },
        Segment::POINTER => {
            if *index == 0 {
                writeln!(s, "@THIS")?;
            } else if *index == 1 {
                writeln!(s, "@THAT")?;
            }
            writeln!(s, "D=A")?;
        },
        Segment::STATIC => {
            writeln!(s, "@16")?;
            writeln!(s, "D=A")?;
            writeln!(s, "@{}", index)?;
            writeln!(s, "D=D+A")?;
        }
    }
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
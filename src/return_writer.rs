use std::fmt::{Error, Write};

pub fn write() -> Result<String, Error> {
    let mut s = String::new();
    // FRAME = LCL
    writeln!(s, "@LCL")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@FRAME")?;
    writeln!(s, "M=D")?;

    // RET = *(FRAME-5)
    writeln!(s, "@FRAME")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@5")?;
    writeln!(s, "D=D-A")?;
    writeln!(s, "@RET")?;
    writeln!(s, "M=D")?;

    // *ARG = pop()
    writeln!(s, "@ARG")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@TMP")?;
    writeln!(s, "M=D")?;
    writeln!(s, "@SP")?;
    writeln!(s, "M=M-1")?;
    writeln!(s, "@SP")?;
    writeln!(s, "A=M")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@TMP")?;
    writeln!(s, "A=M")?;
    writeln!(s, "M=D")?;

    // SP = ARG+1
    writeln!(s, "@ARG")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@SP")?;
    writeln!(s, "M=D+1")?;

    // THAT = *(FRAME-1)
    restore_caller_symbol(&mut s, "THAT", 1)?;

    // THIS = *(FRAME-2)
    restore_caller_symbol(&mut s, "THIS", 2)?;

    // ARG = *(FRAME-3)
    restore_caller_symbol(&mut s, "ARG", 3)?;

    // LCL = *(FRAME-4)
    restore_caller_symbol(&mut s, "LCL", 4)?;

    // goto RET
    writeln!(s, "@RET")?;
    writeln!(s, "A=M")?;
    writeln!(s, "0;JMP")?;

    Ok(s)
}

fn restore_caller_symbol(s: &mut String, symbol: &str, index: i32) -> Result<(), Error> {
    writeln!(s, "@FRAME")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@{}", index)?;
    writeln!(s, "D=D-A")?;
    writeln!(s, "A=D")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@{}", symbol)?;
    writeln!(s, "M=D")?;
    Ok(())
}

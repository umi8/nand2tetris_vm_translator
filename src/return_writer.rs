use std::fmt::{Error, Write};

pub fn write() -> Result<String, Error> {
    let mut s = String::new();
    // FRAME = LCL
    writeln!(s, "@LCL")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@R14")?;
    writeln!(s, "M=D")?;

    // RET = *(FRAME-5)
    writeln!(s, "@R14")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@5")?;
    writeln!(s, "D=D-A")?;
    writeln!(s, "@R15")?;
    writeln!(s, "M=D")?;

    // *ARG = pop()
    writeln!(s, "@ARG")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@R13")?;
    writeln!(s, "M=D")?;
    writeln!(s, "@SP")?;
    writeln!(s, "M=M-1")?;
    writeln!(s, "@SP")?;
    writeln!(s, "A=M")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@R13")?;
    writeln!(s, "A=M")?;
    writeln!(s, "M=D")?;

    // SP = ARG+1
    writeln!(s, "@ARG")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@SP")?;
    writeln!(s, "M=D+1")?;

    // THAT = *(FRAME-1)
    writeln!(s, "@R14")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@1")?;
    writeln!(s, "D=D-A")?;
    writeln!(s, "A=D")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@THAT")?;
    writeln!(s, "M=D")?;

    // THIS = *(FRAME-2)
    writeln!(s, "@R14")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@2")?;
    writeln!(s, "D=D-A")?;
    writeln!(s, "A=D")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@THIS")?;
    writeln!(s, "M=D")?;

    // ARG = *(FRAME-3)
    writeln!(s, "@R14")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@3")?;
    writeln!(s, "D=D-A")?;
    writeln!(s, "A=D")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@ARG")?;
    writeln!(s, "M=D")?;

    // LCL = *(FRAME-4)
    writeln!(s, "@R14")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@4")?;
    writeln!(s, "D=D-A")?;
    writeln!(s, "A=D")?;
    writeln!(s, "D=M")?;
    writeln!(s, "@LCL")?;
    writeln!(s, "M=D")?;

    // goto RET
    writeln!(s, "@R15")?;
    writeln!(s, "A=M")?;
    writeln!(s, "0;JMP")?;

    Ok(s)
}

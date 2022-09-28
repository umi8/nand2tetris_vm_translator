pub fn add() -> String {
    format!("@257\nD=M\n@256\nM=M+D")
}

#[cfg(test)]
mod tests {
    use crate::arithmetic_writer::*;

    #[test]
    fn can_add() {
        assert_eq!(add(), "@257\nD=M\n@256\nM=M+D");
    }
}

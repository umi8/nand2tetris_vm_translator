pub(crate) fn add(stack_pointer: i32) -> String {
    let first = stack_pointer - 2;
    let second = stack_pointer - 1;
    format!("@{}\nD=M\n@{}\nM=M+D", second, first)
}

fn sub(stack_pointer: i32) -> String {
    let first = stack_pointer - 2;
    let second = stack_pointer - 1;
    format!("@{}\nD=M\n@{}\nM=M-D", second, first)
}

fn neg(stack_pointer: i32) -> String {
    let target = stack_pointer - 1;
    format!("@{}\nM=!M", target)
}

#[cfg(test)]
mod tests {
    use crate::arithmetic_writer::{add, neg, sub};

    #[test]
    fn can_add() {
        assert_eq!(add(258), "@257\nD=M\n@256\nM=M+D");
    }

    #[test]
    fn can_sub() {
        assert_eq!(sub(258), "@257\nD=M\n@256\nM=M-D");
    }

    #[test]
    fn can_neg() {
        assert_eq!(neg(257), "@256\nM=!M");
    }
}

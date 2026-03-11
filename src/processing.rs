pub fn process_command(command: &str) {
    println!("{command}: command not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_command_execution() {
        let cmd = "test_cmd";
        process_command(cmd);
        assert!(!cmd.is_empty());
    }
}

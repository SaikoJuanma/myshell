use std::collections::HashMap;
use std::process::exit;

type CommandFn = fn(&str) -> String;

pub fn process_command(input: &str) {
    let mut commands: HashMap<&str, CommandFn> = HashMap::new();

    commands.insert("exit", exit_cmd);
    commands.insert("echo", echo_cmd);

    let parts: Vec<&str> = input.split_whitespace().collect();
    if let Some(cmd_name) = parts.first() {
        if let Some(func) = commands.get(cmd_name) {
            let output = func(input);
            if output == "EXIT" {
                exit(0);
            } else {
                println!("{}", output);
            }
        } else {
            println!("{}: command not found", cmd_name);
        }
    }
}

fn exit_cmd(_args: &str) -> String {
    "EXIT".to_string()
}

fn echo_cmd(args: &str) -> String {
    let echo: String = args
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ");
    echo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_cmd_functionality() {
        // We pass the full string, just like the real app would
        let input = "echo hello world";
        let result = echo_cmd(input);

        // This is a direct test of the function logic
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_echo_cmd_empty() {
        let input = "echo";
        let result = echo_cmd(input);

        assert_eq!(result, "");
    }
}

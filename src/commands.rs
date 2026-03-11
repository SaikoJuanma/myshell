use std::collections::HashMap;
use std::process::exit;

pub struct ShellState {
    pub builtin_names: Vec<&'static str>,
}

type CommandFn = fn(&ShellState, &str) -> String;

pub struct Shell {
    state: ShellState,
    commands: HashMap<&'static str, CommandFn>,
}

impl Shell {
    pub fn new() -> Shell {
        let commands: HashMap<&'static str, CommandFn> = [
            ("echo", echo_cmd as CommandFn),
            ("exit", exit_cmd as CommandFn),
            ("type", type_cmd as CommandFn),
        ]
        .into();

        let builtin_names = commands.keys().copied().collect();

        Shell {
            state: ShellState { builtin_names },
            commands,
        }
    }

    pub fn process_command(&self, input: &str) {
        let parts: Vec<&str> = input.split_whitespace().collect();

        let Some(cmd_name) = parts.first() else {
            return;
        };

        if let Some(func) = self.commands.get(cmd_name) {
            let output = func(&self.state, input);

            if output == "EXIT" {
                exit(0);
            } else if !output.is_empty() {
                println!("{}", output);
            }
        } else {
            println!("{}: command not found", cmd_name);
        }
    }
}

fn echo_cmd(_state: &ShellState, args: &str) -> String {
    args.split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ")
}

fn exit_cmd(_state: &ShellState, _args: &str) -> String {
    "EXIT".to_string()
}

fn type_cmd(state: &ShellState, args: &str) -> String {
    let Some(name) = args.split_whitespace().nth(1) else {
        return "type: missing argument".to_string();
    };

    if state.builtin_names.contains(&name) {
        return format!("{} is a shell builtin", name);
    }

    // TODO Stage 7: search $PATH for external executables here

    format!("{}: not found", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state() -> ShellState {
        ShellState {
            builtin_names: vec!["echo", "exit", "type"],
        }
    }

    #[test]
    fn test_echo_cmd_functionality() {
        let state = make_state();
        let result = echo_cmd(&state, "echo hello world");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_echo_cmd_empty() {
        let state = make_state();
        let result = echo_cmd(&state, "echo");
        assert_eq!(result, "");
    }

    #[test]
    fn test_type_builtin() {
        let state = make_state();
        let result = type_cmd(&state, "type echo");
        assert_eq!(result, "echo is a shell builtin");
    }

    #[test]
    fn test_type_not_found() {
        let state = make_state();
        let result = type_cmd(&state, "type foobar");
        assert_eq!(result, "foobar: not found");
    }

    #[test]
    fn test_type_missing_argument() {
        let state = make_state();
        let result = type_cmd(&state, "type");
        assert_eq!(result, "type: missing argument");
    }
}

mod builtin;
mod signal_handler;
mod test_lab;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::{env, process};

const VERSION_MAJOR: i32 = 1;
const VERSION_MINOR: i32 = 0;
const BUILTINS: [&str; 4] = ["pwd", "cd", "history", "exit"];

/// Prints the usage message.
fn print_help() {
    println!("Usage: {} [-v]", env::args().nth(0).unwrap());
}

/// Retrieves the shell prompt from the environment variable `MY_PROMPT`.
/// If the variable is not set, returns the default prompt "shell> ".
///
/// # Returns
/// A `String` containing the prompt.
fn get_prompt() -> String {
    match env::var_os("MY_PROMPT") {
        Some(val) => {
            val.into_string()
                .expect("error converting prompt to Unicode")
                + " "
        }
        None => "shell> ".to_string(),
    }
}

/// Handles command-line arguments passed to the shell.
/// If the `-v` argument is provided, prints the version and exits.
/// If an invalid argument is provided, prints an error message and exits.
fn handle_args() {
    let a1 = env::args().nth(1);
    let a2 = env::args().nth(2);
    match (a1, a2) {
        (Some(s), None) if s == "-v" => {
            println!("version: {}.{}", VERSION_MAJOR, VERSION_MINOR);
            process::exit(0);
        }
        (Some(_), _) => {
            println!("invalid argument");
            print_help();
            process::exit(1);
        }
        _ => {}
    }
}

/// Parses a command line string into a command and its arguments.
///
/// # Arguments
/// * `line` - A string pointer to the command line input.
///
/// # Returns
/// A tuple containing the command as a `&str` and the arguments as a `Vec<&str>`.

fn parse_cmd(line: &str) -> (&str, Vec<&str>) {
    let mut iter = line.split_whitespace();
    let cmd_option = iter.next();
    let cmd = cmd_option.unwrap_or("");
    let args = iter.collect();
    (cmd, args)
}

/// Executes a built-in command with the provided arguments.
///
/// # Arguments
/// * `cmd` - A string pointer to the command.
/// * `args` - A vector of string pointers to the arguments.
fn run_builtin(cmd: &str, args: Vec<&str>) {
    match cmd {
        "pwd" => {
            if args.len() != 0 {
                println!("usage: pwd");
            } else {
                let result = builtin::pwd();
                match result {
                    Ok(path) => println!("{}", path.display()),
                    Err(e) => eprintln!("{}", e)
                }
            }
        }
        "cd" => {
            if args.len() > 1 {
                println!("usage: cd [dir]");
            } else {
                // if no argument is provided, cd to the home directory
                let result = builtin::cd(if args.len() == 0 {"~"} else {&args[0]});
                match result {
                    Err(e) => eprintln!("{}", e),
                    _ => ()
                }
            }
        }
        "exit" => {
            if args.len() != 0 {
                println!("usage: exit");
            } else {
                builtin::exit()
            }
        }
        // the history command should be handled in main, it will only reach here if ran incorrectly
        "history" => println!("usage: history"),
        _ => ()
    }
}

/// The main function of the shell program.
/// Handles command-line arguments, initializes the readline editor, and enters the main loop
/// to read and execute commands. When a child process is running, the shell will ignore a set of signals.
/// The shell supports built-in commands `pwd`, `cd`, `history`, and `exit`.
/// Other commands are executed as separate processes. The shell also supports a custom prompt
/// set by the `MY_PROMPT` environment variable. The shell will save command history to a file `history.txt`.
fn main() {
    handle_args();

    let mut rl = DefaultEditor::new().expect("error creating editor");
    let _ = rl.load_history("history.txt");
    let prompt = get_prompt();

    // the signal handler will run in a new thread
    let signal_handler = signal_handler::SignalHandler::new();
    signal_handler.start();

    loop {
        let readline = rl.readline(prompt.as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())
                    .expect("error adding history");

                // history command is handled here because it requires ownership of rl
                if line == "history" {
                    for (i, entry) in rl.history().iter().enumerate() {
                        println!("{}: {}", i, entry);
                    }
                    continue;
                }

                let (cmd, args) = parse_cmd(line.as_str());
                if cmd.is_empty() {
                    continue;
                }

                // run if the command is a builtin command
                if BUILTINS.contains(&cmd) {
                    run_builtin(cmd, args);
                    continue;
                }

                // spawn a new process for non-builtin commands
                // ignore signals in parent while the child is running
                signal_handler.ignore_signals(true);
                let status = process::Command::new(cmd).args(&args).status();
                signal_handler.ignore_signals(false);

                // print error if the command failed
                // this will also catch cases where args is too long
                match status {
                    Err(e) => eprintln!("{}", e),
                    _ => (),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("ctrl-c");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("ctrl-d");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")
        .expect("error saving history");
}

/* ---------------------------------------------------------------------------------------
   These wrapper functions allow the testing of private functions from a different module
   These function will only be compiled when testing because of the #[cfg(test)] attribute
   --------------------------------------------------------------------------------------- */
#[cfg(test)]
pub(crate) fn test_wrapper_get_prompt() -> String {
    get_prompt()
}

#[cfg(test)]
pub(crate) fn test_wrapper_parse_cmd(line: &str) -> (&str, Vec<&str>) {
    parse_cmd(line)
}

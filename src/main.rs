use std::io::{self, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    loop {
        print!("\x1b[32m>>\x1b[0m ");
        io::stdout().flush()?;

        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        if input.trim().is_empty() {
            continue;
        }

        if input.trim() == "exit" {
            break;
        }

        let parts = input
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();

        let command = &parts[0];
        let args = &parts[1..];

        let status = Command::new(command)
            .args(args)
            .status();

        match status {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Command `{}` finished with errors!", command);
                }
            }
            Err(_e) => {
                eprintln!("Command `{}` failed with error: {}", command, _e);
            }
        }
    }
    Ok(())
}

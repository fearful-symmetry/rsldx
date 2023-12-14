mod cli;

use anyhow::Result;
use clap::Parser;
use cli::Cli;


fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut dev = linedisplayrs::find_display()?;

    match cli.command {
        cli::Commands::Info => {println!("{}", dev)},
        cli::Commands::Reset => {dev.reset_display()?},
        cli::Commands::Print { message } => {
            dev.write_str(&handle_escape_strings(message))?
        },
        cli::Commands::Cursor(mode) => {
            dev.cursor(mode.into())?
        },
        cli::Commands::Scroll { message, direction, position} => {
            dev.write_str_scroll(&message, direction.into(), position.into())?
        },
        cli::Commands::Smart { message, direction, position } => {
            dev.write_str_smart_scroll(message, direction.into(), position.into())?
        }
    }

    Ok(())
} 

/// parse an escaped newline as the CRLF expected by the terminal
fn handle_escape_strings(input: String) -> String {
    input.replace("\\n", "\r\n")
}
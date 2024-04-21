pub mod cli;
pub mod commands;
pub mod memo;

use cli::cli;
use commands::{MemoCommand, MemoCommandHandler};
use memo::Memo;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut memo = Memo::get_default().expect("Could not load memo file");
    let mut command = MemoCommandHandler { memo: &mut memo };

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some((MemoCommand::ADD, sub_matches)) => {
            let key: &String = sub_matches.get_one("KEY").unwrap();
            let value: &String = sub_matches.get_one("VALUE").unwrap();
            command.add(key, value);
        }

        Some((MemoCommand::GET, sub_matches)) => {
            let key: &String = sub_matches.get_one("KEY").expect("KEY is required");
            let to_clipboard = sub_matches.get_flag("clipboard");
            command.get(key, to_clipboard)?;
        }

        Some((MemoCommand::RM, sub_matches)) => {
            let key: &String = sub_matches.get_one("KEY").expect("KEY is required");
            command.rm(key)?;
        }
        Some((MemoCommand::LIST, _)) => command.list(),

        _ => {
            println!("No subcommand found");
        }
    }
    Ok(())
}

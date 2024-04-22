pub mod cli;
pub mod commands;
pub mod memo;

use cli::{cli, MemoArg};
use commands::{MemoCommand, MemoCommandHandler};
use memo::Memo;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let mut memo = Memo::get_default().expect("Could not load memo file");
    memo.flush_ttl_values()?;

    let mut command = MemoCommandHandler { memo: &mut memo };

    match cli().get_matches().subcommand() {
        Some((MemoCommand::ADD, sub_matches)) => {
            let key: &String = sub_matches.get_one(MemoArg::KEY).unwrap();
            let value: &String = sub_matches.get_one(MemoArg::VALUE).unwrap();
            let ttl: Option<i64> = sub_matches
                .get_one(MemoArg::TTL)
                .map(|t| chrono::Utc::now().timestamp() + t);

            command.add(key, value, ttl);
        }

        Some((MemoCommand::GET, sub_matches)) => {
            let key: &String = sub_matches.get_one(MemoArg::KEY).expect("KEY is required");
            let to_clipboard = sub_matches.get_flag("clipboard");
            command.get(key, to_clipboard)?;
        }

        Some((MemoCommand::RM, sub_matches)) => {
            let key: &String = sub_matches.get_one(MemoArg::KEY).expect("KEY is required");
            command.rm(key)?;
        }
        Some((MemoCommand::LIST, _)) => command.list(),
        Some((MemoCommand::SET, sub_matches)) => {
            let key: &String = sub_matches.get_one(MemoArg::KEY).unwrap();

            let ttl: Option<i64> = sub_matches
                .get_one(MemoArg::TTL)
                .map(|t| chrono::Utc::now().timestamp() + t);
            let value: Option<&String> = sub_matches.get_one(MemoArg::VALUE);
            command.set(key, value.map(|v| v.as_str()), ttl);
        }

        _ => {
            println!("No subcommand found");
        }
    }
    Ok(())
}

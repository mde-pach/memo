use crate::commands::MemoCommand;
use clap::{arg, Command};

pub fn cli() -> Command {
    let add = Command::new(MemoCommand::ADD)
        .about("Add a new memo")
        .arg(arg!(<KEY> "New item key to add."))
        .arg(arg!(<VALUE> "New item value to add."));

    let get = Command::new(MemoCommand::GET)
        .about("Get a memo")
        .arg(arg!(<KEY> "The key of the item to get."))
        .arg(arg!(-c --clipboard  "Copy the value to the clipboard."));

    let rm = Command::new(MemoCommand::RM)
        .about("Remove a memo")
        .arg(arg!(<KEY> "The key of the item to remove."));

    let list = Command::new(MemoCommand::LIST).about("List all memos");

    Command::new("memo")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .subcommand(add)
        .subcommand(get)
        .subcommand(rm)
        .subcommand(list)
}

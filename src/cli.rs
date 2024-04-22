use crate::commands::MemoCommand;
use clap::{arg, Arg, Command};

pub struct MemoArg;

impl MemoArg {
    pub const KEY: &'static str = "KEY";
    pub const VALUE: &'static str = "VALUE";
    pub const TTL: &'static str = "ttl";
}
/// Handles the CLI commands for the application.
pub fn cli() -> Command {
    let ttl_arg = Arg::new("ttl")
        .short('t')
        .long("ttl")
        .help("Time to live for the item in seconds.")
        .value_parser(clap::value_parser!(i64).range(0..=i64::MAX));

    let add = Command::new(MemoCommand::ADD)
        .about("Add a new memo")
        .arg(arg!(<KEY> "New item key to add."))
        .arg(arg!(<VALUE> "New item value to add."))
        .arg(&ttl_arg);

    let get = Command::new(MemoCommand::GET)
        .about("Get a memo")
        .arg(arg!(<KEY> "The key of the item to get."))
        .arg(arg!(-c --clipboard  "Copy the value to the clipboard."));

    let rm = Command::new(MemoCommand::RM)
        .about("Remove a memo")
        .arg(arg!(<KEY> "The key of the item to remove."));

    let list = Command::new(MemoCommand::LIST).about("List all memos");

    let set: Command = Command::new(MemoCommand::SET)
        .about("Set a memo")
        .arg(arg!(<KEY> "Item key to set."))
        .arg(arg!([VALUE] "Item value to set."))
        .arg(&ttl_arg);

    Command::new("memo")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .subcommand(add)
        .subcommand(get)
        .subcommand(rm)
        .subcommand(list)
        .subcommand(set)
}

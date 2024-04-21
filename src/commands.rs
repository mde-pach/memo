use crate::memo::Memo;
use arboard::Clipboard;
use std::error::Error;
pub struct MemoCommand;

impl MemoCommand {
    pub const ADD: &'static str = "add";
    pub const GET: &'static str = "get";
    pub const RM: &'static str = "rm";
    pub const LIST: &'static str = "list";
}
pub struct MemoCommandHandler<'a> {
    pub memo: &'a mut Memo,
}

impl MemoCommandHandler<'_> {
    pub fn add(&mut self, key: &str, value: &str) {
        match self.memo.get(key) {
            Some(_) => {
                println!("Key already exists: {}", key);
            }
            None => match self.memo.set(key, value) {
                Ok(_) => {
                    println!("Added key: {}", key);
                }
                Err(e) => {
                    println!("Error adding key: {}", e);
                }
            },
        }
    }

    pub fn get(&self, key: &str, to_clipboard: bool) -> Result<(), Box<dyn Error>> {
        match self.memo.get(key) {
            Some(v) => {
                let value = &v.value;
                if to_clipboard {
                    let mut clipboard = Clipboard::new()?;
                    clipboard.set_text(value)?;
                }
                println!("{}", value);
            }
            None => {
                println!("No value found for key: {}", key);
            }
        }
        Ok(())
    }
    pub fn rm(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        match self.memo.get(key) {
            Some(_) => {
                self.memo.rm(key)?;
                println!("Removing key: {}", key);
            }
            None => {
                println!("No value found for key: {}", key);
            }
        }
        Ok(())
    }

    pub fn list(&self) {
        for (key, value) in &self.memo.store {
            println!("{} : {}", key, value.value);
        }
    }
}

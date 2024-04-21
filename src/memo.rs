use home::home_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct MemoVariable {
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Memo {
    pub store: HashMap<String, MemoVariable>,
    file_path: PathBuf,
}

impl Memo {
    pub fn get(&self, key: &str) -> Option<&MemoVariable> {
        self.store.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.store.insert(
            key.to_string(),
            MemoVariable {
                value: value.to_string(),
            },
        );
        serde_json::to_writer_pretty(&File::create(&self.file_path)?, &self.store)?;
        Ok(())
    }
    pub fn rm(&mut self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.store.remove(key);
        serde_json::to_writer_pretty(&File::create(&self.file_path)?, &self.store)?;
        Ok(())
    }

    pub fn from_file_path(file_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(&file_path)?;

        let memo = Self {
            store: serde_json::from_reader(file)?,
            file_path,
        };
        Ok(memo)
    }

    pub fn get_default() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = Self::ensure_directory_and_file(Self::get_memo_dir()?, "default.json")?;
        Ok(Memo::from_file_path(file_path)?)
    }

    fn ensure_directory_and_file(
        directory_path: PathBuf,
        filename: &str,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        if !directory_path.exists() {
            fs::create_dir_all(&directory_path)?;
        }

        let file_path = directory_path.join(filename);

        if !file_path.exists() {
            File::create(&file_path)?;
        }

        Ok(file_path)
    }

    fn get_memo_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let home_dir = home_dir()
            .ok_or("Could not find home directory")?
            .join(".memo");
        Ok(home_dir)
    }
}

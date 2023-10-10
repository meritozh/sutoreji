// Copyright (c) 2023 meritozh
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mime_guess::Mime;
use tokio::{fs::read_dir, io};
use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug)]
pub enum CmdError {
    #[error("std filesystem module error")]
    Io(#[from] io::Error),
}

impl Serialize for CmdError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
pub async fn get_dir_list(path: String) -> Result<Vec<String>, CmdError> {
    let mut reader = read_dir(path).await?;
    let mut file_list = vec![];
    while let Some(entry) = reader.next_entry().await? {
        if let Some(name) = entry.path().file_name() {
            if let Some(name) = name.to_str().map(str::to_string) {
                file_list.push(name);
            }
        }
    }
    Ok(file_list)
}

#[derive(Debug, Serialize)]
enum EntryType {
    Folder,
    File(String)
}

#[derive(Debug, Serialize)]
struct EntryInfo {
    type_: EntryType,
    name: String
}
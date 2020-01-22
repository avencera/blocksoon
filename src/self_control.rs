use chrono::{DateTime, Utc};
use dirs;
use plist;
use serde::Deserialize;
use std::option::Option;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SelfControl {
    pub block_duration: f64,
    pub block_started_date: DateTime<Utc>,
    pub host_blacklist: Vec<String>,
}

impl SelfControl {
    pub fn new() -> Option<SelfControl> {
        let home_dir = dirs::home_dir()?;
        let file_path = format!(
            "{}/Library/Preferences/org.eyebeam.SelfControl.plist",
            home_dir.to_str()?
        );

        plist::from_file(&file_path).ok()
    }
}

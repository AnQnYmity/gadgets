use chrono::{DateTime, Local};
use std::time::SystemTime;

pub fn format_time_stamp(ref time: SystemTime) -> String {
    let datetime: DateTime<Local> = (*time).into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
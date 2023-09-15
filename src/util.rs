use chrono::{DateTime, Utc, Local};

pub fn date_fmt() -> String {
  let dt =  Utc::now();
  let local_dt: DateTime<Local> = dt.with_timezone(&Local);
  local_dt.format("%Y-%m-%d %H:%M:%S").to_string()
}
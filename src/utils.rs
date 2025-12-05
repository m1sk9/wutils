use chrono::NaiveTime;

pub struct ParseTimeArgs {
    pub time_str: String,
}

impl ParseTimeArgs {
    pub fn parse_time(&self) -> Result<NaiveTime, String> {
        NaiveTime::parse_from_str(&self.time_str, "%H:%M")
            .or_else(|_| NaiveTime::parse_from_str(&self.time_str, "%H:%M:%S"))
            .map_err(|e| format!("Failed to parse time '{}': {}", &self.time_str, e))
    }
}

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

    /// Parse duration string (HH:MM or HH:MM:SS) to total seconds.
    /// Unlike parse_time(), this supports hours >= 24.
    pub fn parse_duration(&self) -> Result<u64, String> {
        let parts: Vec<&str> = self.time_str.split(':').collect();

        match parts.len() {
            2 => {
                let hours: u64 = parts[0]
                    .parse()
                    .map_err(|_| format!("Failed to parse hours '{}'", parts[0]))?;
                let minutes: u64 = parts[1]
                    .parse()
                    .map_err(|_| format!("Failed to parse minutes '{}'", parts[1]))?;

                if minutes >= 60 {
                    return Err(format!("Minutes '{}' must be less than 60", minutes));
                }

                Ok(hours * 3600 + minutes * 60)
            }
            3 => {
                let hours: u64 = parts[0]
                    .parse()
                    .map_err(|_| format!("Failed to parse hours '{}'", parts[0]))?;
                let minutes: u64 = parts[1]
                    .parse()
                    .map_err(|_| format!("Failed to parse minutes '{}'", parts[1]))?;
                let seconds: u64 = parts[2]
                    .parse()
                    .map_err(|_| format!("Failed to parse seconds '{}'", parts[2]))?;

                if minutes >= 60 {
                    return Err(format!("Minutes '{}' must be less than 60", minutes));
                }
                if seconds >= 60 {
                    return Err(format!("Seconds '{}' must be less than 60", seconds));
                }

                Ok(hours * 3600 + minutes * 60 + seconds)
            }
            _ => Err(format!(
                "Invalid time format '{}': expected HH:MM or HH:MM:SS",
                self.time_str
            )),
        }
    }
}

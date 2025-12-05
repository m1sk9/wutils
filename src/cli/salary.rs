use crate::utils::ParseTimeArgs;
use chrono::Timelike;
use std::process::exit;

pub struct SalaryArgs {
    pub total_work_hours: String,
    pub hourly_rate: f64,
}

impl SalaryArgs {
    pub fn execute(&self) {
        let time = ParseTimeArgs {
            time_str: self.total_work_hours.clone(),
        };

        let parsed_time = time.parse_time().unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            exit(1);
        });

        let total_seconds = parsed_time.num_seconds_from_midnight() as f64;
        let total_hours = total_seconds / 3600.0;
        let total_salary = total_hours * self.hourly_rate;

        println!("Total work time: {}", self.total_work_hours);
        println!("Total salary: {:.0}", total_salary);
    }
}

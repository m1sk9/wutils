use crate::utils::ParseTimeArgs;
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

        let total_seconds = time.parse_duration().unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            exit(1);
        });

        let total_hours = total_seconds as f64 / 3600.0;
        let total_salary = total_hours * self.hourly_rate;

        println!("Total work time: {}", self.total_work_hours);
        println!("Total salary: {:.0}", total_salary);
    }
}

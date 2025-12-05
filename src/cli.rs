pub mod salary;
pub mod time;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wutils", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adjust end time based on breaks taken.
    Time {
        /// End time in HH:MM or HH:MM:SS format.
        end_time: String,
        /// List of break start and end times in HH:MM or HH:MM:SS format.
        #[arg(long = "break", num_args = 2, action = clap::ArgAction::Append,
            value_names = ["START", "END"], required = true)]
        breaks: Vec<String>,
    },
    /// Calculate total salary from work hours and hourly rate.
    Salary {
        /// Total work hours in HH:MM or HH:MM:SS format.
        total_work_hours: String,
        /// Hourly rate in yen.
        hourly_rate: f64,
    },
}

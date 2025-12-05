use clap::Parser;

use crate::cli::{Cli, salary::SalaryArgs, time::TimeArgs};

mod cli;
mod utils;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Time { end_time, breaks } => {
            let args = TimeArgs { end_time, breaks };
            args.execute();
        }
        cli::Commands::Salary {
            total_work_hours,
            hourly_rate,
        } => {
            let args = SalaryArgs {
                total_work_hours,
                hourly_rate,
            };
            args.execute();
        }
    }
}

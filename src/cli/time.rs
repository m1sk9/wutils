use std::process::exit;

use chrono::Duration;

use crate::utils::ParseTimeArgs;

pub struct TimeArgs {
    pub end_time: String,
    pub breaks: Vec<String>,
}

impl TimeArgs {
    pub fn execute(&self) -> () {
        let mut end = parse_or_exit(&self.end_time);
        let mut total_break_duration = Duration::zero();

        for b in self.breaks.chunks(2) {
            if b.len() != 2 {
                eprintln!("Each break must have a start and end time.");
                exit(2);
            }

            let start = parse_or_exit(&b[0]);
            let stop = parse_or_exit(&b[1]);

            if stop < start {
                eprintln!(
                    "Invalid break: end time ({}) is before start time ({}).",
                    b[1], b[0]
                );
                exit(3);
            }

            total_break_duration += stop - start;
        }

        end -= total_break_duration;

        println!(
            "Total break time: {}min",
            total_break_duration.num_minutes()
        );
        println!("Final departure time: {}", end.format("%H:%M"))
    }
}

fn parse_or_exit(input: &str) -> chrono::NaiveTime {
    let args = ParseTimeArgs {
        time_str: input.to_string(),
    };
    args.parse_time().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        exit(1);
    })
}

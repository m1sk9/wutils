# wutils

A collection of useful command-line working utilities written in Rust.

## Installation

A Rust environment is required.

```sh
cargo install wutils
```

## Usage

- `wutils time <scheduled> --break <break_start> <break_end> ...`
  - Calculates the actual working time from the scheduled working hours by excluding the specified break periods.

  ```sh
  wutils time 8:00 --break 12:00 13:00 --break 15:00 15:15
  ```

- `wutils salary <total-working-hours> <hourly-wage>`
  - Calculates wages based on the hourly wage derived from working hours.
  - The hourly wage is either specified as an argument or uses the value set in the configuration.
  
  ```sh
  wutils salary 20:15:15 1300
  ```

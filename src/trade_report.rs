use chrono::Local;
use crate::utils::states::States;
use std::io;
use std::io::Write;

pub fn log_state(ticker: &str, state: States, strategy_name: String, cursor_pos: (u16, u16)) {
    let date_time = Local::now();
    // Move the cursor to the specified position
    print!("\x1B[{};{}H", cursor_pos.0, cursor_pos.1);
    // Clear the line
    print!("\x1B[K");
    // Print the log message
    println!("[{}] - {:?}: - {:?} - {:?}", date_time.format("%Y-%m-%d %H:%M:%S"), ticker, strategy_name, state);
    // Flush the stdout buffer
    io::stdout().flush().unwrap();
}

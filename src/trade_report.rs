use chrono::Local;
use crate::utils::states::States;

pub fn log_state(state: States, model: String) {
    // Getting datetime
    let now = Local::now();
    // Getting func name
    let file = file!();
    let line = line!();
    println!(
        "[{}] File: {}, Line: {} - Model: {} -State: {:?}",
        now.format("%Y-%m-%d %H:%M:%S"),
        file,
        line,
        model,
        state
    );
}
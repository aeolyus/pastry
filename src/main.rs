use std::process::ExitCode;

use pastry;

fn main() -> ExitCode {
    let result = pastry::run();
    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::FAILURE
        }
    }
}

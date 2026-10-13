use std::env;
use std::fs;
use std::process::ExitCode;

use us_core_allergy::validate_allergy_json;

fn main() -> ExitCode {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: validate-allergy <path.json>");
            return ExitCode::from(2);
        }
    };

    let json = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("read {}: {}", path, e);
            return ExitCode::from(2);
        }
    };

    let report = match validate_allergy_json(&json) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(2);
        }
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&report.issues).unwrap_or_else(|_| "[]".into())
    );

    if report.is_valid() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

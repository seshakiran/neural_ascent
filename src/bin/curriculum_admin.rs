#![allow(dead_code)]

#[path = "../state.rs"]
mod state;
#[path = "../levels.rs"]
mod levels;

use levels::{
    export_curriculum_database_to_seed, reseed_curriculum_database, validate_curriculum,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("curriculum_admin error: {}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let command = std::env::args().nth(1).unwrap_or_else(|| "help".to_string());

    match command.as_str() {
        "reseed" => {
            reseed_curriculum_database()?;
            println!("SQLite curriculum reseeded from data/curriculum_seed.json");
        }
        "export" => {
            export_curriculum_database_to_seed()?;
            println!("Seed JSON refreshed from SQLite database");
        }
        "validate" => {
            let findings = validate_curriculum()?;
            if findings.is_empty() {
                println!("Curriculum validation passed");
            } else {
                println!("Curriculum validation findings:");
                for finding in findings {
                    println!("- {}", finding);
                }
                std::process::exit(2);
            }
        }
        _ => {
            println!("Usage: cargo run --bin curriculum_admin -- <reseed|export|validate>");
        }
    }

    Ok(())
}

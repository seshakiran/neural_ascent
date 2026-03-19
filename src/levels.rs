//! CURRICULUM DATA LOADED FROM A LOCAL SQLITE FILE
use crate::state::LessonDepth;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizQuestion {
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: String,
    pub explanation: String,
    pub xp_reward: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub language: String,
    pub title: String,
    pub code: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealWorldExample {
    pub title: String,
    pub description: String,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub title: String,
    pub description: String,
    pub xp_reward: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelStep {
    pub title: String,
    pub foundations: String,
    pub theory: String,
    pub code_examples: Vec<CodeExample>,
    pub real_world_examples: Vec<RealWorldExample>,
    pub quiz: Vec<QuizQuestion>,
    pub challenge: Option<Challenge>,
    pub is_synthesis: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub id: usize,
    pub name: String,
    pub overview: String,
    pub prerequisites: Vec<String>,
    pub steps: Vec<LevelStep>,
}

pub struct LevelManager {
    levels: Vec<Level>,
}

impl LevelManager {
    pub fn new(depth: LessonDepth) -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = curriculum_db_path();
        initialize_database(&db_path)?;

        let level_rows: Vec<LevelRow> = run_sqlite_json(
            &db_path,
            "SELECT id, name, overview, prerequisites_json
             FROM levels
             ORDER BY id;",
        )?;

        let mut levels = Vec::with_capacity(level_rows.len());
        for row in level_rows {
            let steps = load_steps(&db_path, row.id, &depth)?;
            levels.push(Level {
                id: row.id,
                name: row.name,
                overview: row.overview,
                prerequisites: serde_json::from_str(&row.prerequisites_json)?,
                steps,
            });
        }

        Ok(Self { levels })
    }

    pub fn get_level(&self, id: usize) -> &Level {
        &self.levels[id.min(self.levels.len() - 1)]
    }

    pub fn level_count(&self) -> usize {
        self.levels.len()
    }
}

#[allow(dead_code)]
pub fn reseed_curriculum_database() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = curriculum_db_path();
    initialize_database(&db_path)?;
    seed_database(&db_path)?;
    Ok(())
}

#[allow(dead_code)]
pub fn export_curriculum_database_to_seed() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = curriculum_db_path();
    initialize_database(&db_path)?;

    let level_rows: Vec<LevelRow> = run_sqlite_json(
        &db_path,
        "SELECT id, name, overview, prerequisites_json
         FROM levels
         ORDER BY id;",
    )?;

    let mut levels = Vec::with_capacity(level_rows.len());
    for row in level_rows {
        let query = format!(
            "SELECT title, foundations, standard_theory, deep_theory,
                    code_examples_json, real_world_examples_json,
                    quiz_json, challenge_json, is_synthesis
             FROM steps
             WHERE level_id = {}
             ORDER BY step_index;",
            row.id
        );
        let steps: Vec<StepRow> = run_sqlite_json(&db_path, &query)?;
        levels.push(SeedLevel {
            id: row.id,
            name: row.name,
            overview: row.overview,
            prerequisites: serde_json::from_str(&row.prerequisites_json)?,
            steps: steps
                .into_iter()
                .map(|row| SeedStep {
                    title: row.title,
                    foundations: row.foundations,
                    standard_theory: row.standard_theory,
                    deep_theory: row.deep_theory,
                    code_examples: serde_json::from_str(&row.code_examples_json)
                        .unwrap_or_default(),
                    real_world_examples: serde_json::from_str(&row.real_world_examples_json)
                        .unwrap_or_default(),
                    quiz: serde_json::from_str(&row.quiz_json).unwrap_or_default(),
                    challenge: row
                        .challenge_json
                        .and_then(|json| serde_json::from_str(&json).ok()),
                    is_synthesis: row.is_synthesis != 0,
                })
                .collect(),
        });
    }

    let raw = serde_json::to_string_pretty(&levels)?;
    fs::write(seed_json_path(), raw)?;
    Ok(())
}

#[allow(dead_code)]
pub fn validate_curriculum() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let raw = fs::read_to_string(seed_json_path())?;
    let levels: Vec<SeedLevel> = serde_json::from_str(&raw)?;
    let mut findings = Vec::new();

    for level in &levels {
        for step in &level.steps {
            if step.foundations.starts_with("Foundation for level") {
                findings.push(format!(
                    "Level {} '{}' still has generated foundations.",
                    level.id, step.title
                ));
            }
            if step.deep_theory.contains("In practice, the important move is") {
                findings.push(format!(
                    "Level {} '{}' still has placeholder deep-theory text.",
                    level.id, step.title
                ));
            }

            let support_text = format!(
                "{} {} {}",
                step.foundations, step.standard_theory, step.deep_theory
            )
            .to_lowercase();

            for question in &step.quiz {
                for term in glossary_terms() {
                    if question.explanation.to_lowercase().contains(term)
                        && !support_text.contains(term)
                    {
                        findings.push(format!(
                            "Level {} '{}' explanation references '{}' without introducing it in the lesson text.",
                            level.id, step.title, term
                        ));
                    }
                }
            }
        }
    }

    Ok(findings)
}

fn load_steps(
    db_path: &Path,
    level_id: usize,
    depth: &LessonDepth,
) -> Result<Vec<LevelStep>, Box<dyn std::error::Error>> {
    let query = format!(
        "SELECT title, foundations, standard_theory, deep_theory,
                code_examples_json, real_world_examples_json,
                quiz_json, challenge_json, is_synthesis
         FROM steps
         WHERE level_id = {}
         ORDER BY step_index;",
        level_id
    );
    let step_rows: Vec<StepRow> = run_sqlite_json(db_path, &query)?;

    let steps = step_rows
        .into_iter()
        .map(|row| LevelStep {
            title: row.title,
            foundations: row.foundations,
            theory: match depth {
                LessonDepth::Guided => row.standard_theory,
                LessonDepth::DeepDive => row.deep_theory,
            },
            code_examples: serde_json::from_str(&row.code_examples_json).unwrap_or_default(),
            real_world_examples: serde_json::from_str(&row.real_world_examples_json)
                .unwrap_or_default(),
            quiz: serde_json::from_str(&row.quiz_json).unwrap_or_default(),
            challenge: row
                .challenge_json
                .and_then(|json| serde_json::from_str(&json).ok()),
            is_synthesis: row.is_synthesis != 0,
        })
        .collect();

    Ok(steps)
}

fn initialize_database(db_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let bootstrap_sql = "
        CREATE TABLE IF NOT EXISTS levels (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            overview TEXT NOT NULL,
            prerequisites_json TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS steps (
            level_id INTEGER NOT NULL,
            step_index INTEGER NOT NULL,
            title TEXT NOT NULL,
            foundations TEXT NOT NULL,
            standard_theory TEXT NOT NULL,
            deep_theory TEXT NOT NULL,
            code_examples_json TEXT NOT NULL,
            real_world_examples_json TEXT NOT NULL,
            quiz_json TEXT NOT NULL,
            challenge_json TEXT,
            is_synthesis INTEGER NOT NULL DEFAULT 0,
            PRIMARY KEY (level_id, step_index)
        );
    ";
    run_sqlite_script(db_path, bootstrap_sql)?;

    let count_rows: Vec<CountRow> = run_sqlite_json(
        db_path,
        "SELECT COUNT(*) AS count FROM levels;",
    )?;
    let has_content = count_rows.first().map(|row| row.count).unwrap_or(0) > 0;

    if !has_content {
        seed_database(db_path)?;
    }

    Ok(())
}

fn seed_database(db_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let seed_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("curriculum_seed.json");
    let raw = fs::read_to_string(seed_path)?;
    let levels: Vec<SeedLevel> = serde_json::from_str(&raw)?;

    let mut script = String::from("BEGIN TRANSACTION;\nDELETE FROM steps;\nDELETE FROM levels;\n");
    for level in levels {
        script.push_str(&format!(
            "INSERT INTO levels (id, name, overview, prerequisites_json) VALUES ({}, {}, {}, {});\n",
            level.id,
            sql_string(&level.name),
            sql_string(&level.overview),
            sql_string(&serde_json::to_string(&level.prerequisites)?),
        ));

        for (step_index, step) in level.steps.into_iter().enumerate() {
            script.push_str(&format!(
                "INSERT INTO steps (
                    level_id, step_index, title, foundations, standard_theory, deep_theory,
                    code_examples_json, real_world_examples_json, quiz_json, challenge_json, is_synthesis
                 ) VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {});\n",
                level.id,
                step_index,
                sql_string(&step.title),
                sql_string(&step.foundations),
                sql_string(&step.standard_theory),
                sql_string(&step.deep_theory),
                sql_string(&serde_json::to_string(&step.code_examples)?),
                sql_string(&serde_json::to_string(&step.real_world_examples)?),
                sql_string(&serde_json::to_string(&step.quiz)?),
                step.challenge
                    .map(|challenge| sql_string(&serde_json::to_string(&challenge).unwrap()))
                    .unwrap_or_else(|| "NULL".to_string()),
                if step.is_synthesis { 1 } else { 0 },
            ));
        }
    }
    script.push_str("COMMIT;\n");
    run_sqlite_script(db_path, &script)?;
    Ok(())
}

fn sql_string(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

fn run_sqlite_script(db_path: &Path, script: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("sqlite3")
        .arg(db_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(script.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(format!(
            "sqlite3 failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}

fn run_sqlite_json<T: for<'de> Deserialize<'de>>(
    db_path: &Path,
    query: &str,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let output = Command::new("sqlite3")
        .arg("-json")
        .arg(db_path)
        .arg(query)
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "sqlite3 query failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    if stdout.trim().is_empty() {
        return Ok(vec![]);
    }

    Ok(serde_json::from_str(&stdout)?)
}

fn curriculum_db_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("curriculum.sqlite")
}

#[allow(dead_code)]
fn seed_json_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("curriculum_seed.json")
}

#[allow(dead_code)]
fn glossary_terms() -> &'static [&'static str] {
    &[
        "loss function",
        "gradient",
        "embedding",
        "retrieval",
        "prompt injection",
        "nonlinearity",
        "parameter",
        "attention",
        "context",
        "drift",
        "calibration",
        "governance",
        "checkpointing",
    ]
}

#[derive(Debug, Deserialize, Serialize)]
struct SeedLevel {
    id: usize,
    name: String,
    overview: String,
    prerequisites: Vec<String>,
    steps: Vec<SeedStep>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SeedStep {
    title: String,
    foundations: String,
    standard_theory: String,
    deep_theory: String,
    code_examples: Vec<CodeExample>,
    real_world_examples: Vec<RealWorldExample>,
    quiz: Vec<QuizQuestion>,
    challenge: Option<Challenge>,
    is_synthesis: bool,
}

#[derive(Debug, Deserialize)]
struct LevelRow {
    id: usize,
    name: String,
    overview: String,
    prerequisites_json: String,
}

#[derive(Debug, Deserialize)]
struct StepRow {
    title: String,
    foundations: String,
    standard_theory: String,
    deep_theory: String,
    code_examples_json: String,
    real_world_examples_json: String,
    quiz_json: String,
    challenge_json: Option<String>,
    is_synthesis: i64,
}

#[derive(Debug, Deserialize)]
struct CountRow {
    count: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_curriculum_from_sqlite() {
        let manager = LevelManager::new(LessonDepth::Guided).expect("sqlite curriculum loads");
        assert!(manager.level_count() >= 25);
        let first_real_level = manager.get_level(1);
        assert!(!first_real_level.steps.is_empty());
        assert!(!first_real_level.steps[0].foundations.trim().is_empty());
    }
}

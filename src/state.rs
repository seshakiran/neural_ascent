//! ======================================================================================
//! STATE MODULE - Persistent Game State Management
//! ======================================================================================

use serde::{Deserialize, Serialize};

/// Difficulty levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Insane,
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::Normal
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LessonDepth {
    Guided,
    DeepDive,
}

impl Default for LessonDepth {
    fn default() -> Self {
        LessonDepth::Guided
    }
}

/// Player progress in a level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelProgress {
    pub level_id: usize,
    pub steps_completed: Vec<usize>,
    pub quiz_scores: Vec<u32>,
    pub challenges_completed: Vec<usize>,
}

impl LevelProgress {
    pub fn new(level_id: usize) -> Self {
        Self {
            level_id,
            steps_completed: vec![],
            quiz_scores: vec![],
            challenges_completed: vec![],
        }
    }
}

/// Complete player state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerState {
    pub player_name: String,
    pub xp: u64,
    pub total_xp_earned: u64,
    pub current_level: usize,
    pub levels_completed: usize,
    pub difficulty: Difficulty,
    pub level_progress: Vec<LevelProgress>,
    pub recent_quiz_scores: Vec<u32>,
    pub all_quiz_scores: Vec<u32>,
    pub achievements: Vec<String>,
    pub total_time_played: u64,
    pub streak_days: u32,
    pub notes: Vec<String>,
    pub bookmarks: Vec<String>,
    pub prometheus_stability: i32,
    pub prometheus_trust: i32,
    pub review_cycles: u32,
    pub lesson_depth: LessonDepth,
}

impl PlayerState {
    pub fn new() -> Self {
        Self {
            player_name: "Apprentice".to_string(),
            xp: 0,
            total_xp_earned: 0,
            current_level: 0,
            levels_completed: 0,
            difficulty: Difficulty::Normal,
            level_progress: vec![LevelProgress::new(0)],
            recent_quiz_scores: vec![],
            all_quiz_scores: vec![],
            achievements: vec![],
            total_time_played: 0,
            streak_days: 0,
            notes: vec![],
            bookmarks: vec![],
            prometheus_stability: 55,
            prometheus_trust: 50,
            review_cycles: 0,
            lesson_depth: LessonDepth::Guided,
        }
    }

    pub fn update_step_progress(&mut self, level_id: usize, step_index: usize) {
        while self.level_progress.len() <= level_id {
            self.level_progress.push(LevelProgress::new(self.level_progress.len()));
        }
        let progress = &mut self.level_progress[level_id];
        if !progress.steps_completed.contains(&step_index) {
            progress.steps_completed.push(step_index);
        }
    }

    pub fn add_quiz_score(&mut self, score: u32) {
        self.all_quiz_scores.push(score);
        self.recent_quiz_scores.push(score);
        if self.recent_quiz_scores.len() > 10 {
            self.recent_quiz_scores.remove(0);
        }
    }

    pub fn add_xp(&mut self, amount: u64) {
        self.xp += amount;
        self.total_xp_earned += amount;
    }

    pub fn apply_quiz_result(&mut self, score: u32, passed: bool) {
        if passed {
            let trust_gain = match score {
                100 => 6,
                85..=99 => 4,
                _ => 2,
            };
            let stability_gain = match score {
                100 => 5,
                85..=99 => 3,
                _ => 1,
            };
            self.prometheus_trust = (self.prometheus_trust + trust_gain).clamp(0, 100);
            self.prometheus_stability =
                (self.prometheus_stability + stability_gain).clamp(0, 100);
        } else {
            self.review_cycles += 1;
            self.prometheus_trust = (self.prometheus_trust - 2).clamp(0, 100);
            self.prometheus_stability = (self.prometheus_stability - 4).clamp(0, 100);
        }
    }

    pub fn apply_level_completion(&mut self) {
        self.prometheus_stability = (self.prometheus_stability + 2).clamp(0, 100);
        self.prometheus_trust = (self.prometheus_trust + 1).clamp(0, 100);
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::new()
    }
}

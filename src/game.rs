//! GAME ENGINE - PROMETHEUS
use crate::levels::{Challenge, LevelManager, LevelStep, QuizQuestion};
use crate::state::PlayerState;
use crate::story::StoryEngine;
use crate::ui::UI;
use std::fs;
use std::path::PathBuf;

const PASS_PERCENT: u32 = 70;

pub struct NeuralAscent {
    player_state: PlayerState,
    level_manager: Option<LevelManager>,
    story_engine: StoryEngine,
    ui: UI,
}

impl NeuralAscent {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            player_state: Self::load_or_create_state()?,
            level_manager: None,
            story_engine: StoryEngine::new(),
            ui: UI::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.ui.clear_screen();
        self.ui.show_prologue()?;

        self.ui.show_message("\n\nWhat is your name, human?");
        let name = self.ui.get_input()?;
        if !name.trim().is_empty() {
            self.player_state.player_name = name.trim().to_string();
        }

        let selected_depth = self.ui.lesson_depth_menu(&self.player_state.lesson_depth)?;
        self.player_state.lesson_depth = selected_depth;
        self.level_manager = Some(LevelManager::new(self.player_state.lesson_depth.clone())?);
        self.save_state()?;

        self.ui.show_stakes()?;

        let level_manager = self.level_manager.as_ref().expect("level manager initialized");
        let level_count = level_manager.level_count();
        let level_choices: Vec<(usize, String)> = (0..level_count)
            .map(|idx| {
                let level = level_manager.get_level(idx);
                (idx, level.name.clone())
            })
            .collect();
        let chosen_level = self
            .ui
            .level_select_menu(self.player_state.current_level.min(level_count.saturating_sub(1)), &level_choices)?;
        self.player_state.current_level = chosen_level;
        self.save_state()?;

        if self.player_state.current_level >= level_count {
            self.ui.show_victory(
                &self.player_state.player_name,
                self.player_state.xp,
                self.player_state.prometheus_stability,
                self.player_state.prometheus_trust,
            )?;
            return Ok(());
        }

        let start_level = self.player_state.current_level.min(level_count.saturating_sub(1));

        for level_num in start_level..level_count {
            let level = self
                .level_manager
                .as_ref()
                .expect("level manager initialized")
                .get_level(level_num)
                .clone();
            let story = self
                .story_engine
                .get_level_story(level_num)
                .render_for_player(&self.player_state.player_name);

            self.ui.show_level_intro(
                level_num + 1,
                level_count,
                &self.player_state.player_name,
                level.name.as_str(),
                level.overview.as_str(),
                &level.prerequisites,
                &story,
                self.player_state.prometheus_stability,
                self.player_state.prometheus_trust,
            )?;

            for (step_index, step) in level.steps.iter().enumerate() {
                self.complete_step(level_num, step_index, step)?;
            }

            let bonus = 120 + (level_num as u64 * 20);
            self.player_state.add_xp(bonus);
            self.player_state.apply_level_completion();
            self.player_state.current_level = (level_num + 1).min(level_count);
            self.player_state.levels_completed = self.player_state.current_level;
            self.ui.level_complete(
                level_num + 1,
                bonus,
                self.player_state.xp,
                self.player_state.prometheus_stability,
                self.player_state.prometheus_trust,
            )?;
            self.save_state()?;
        }

        self.ui.show_victory(
            &self.player_state.player_name,
            self.player_state.xp,
            self.player_state.prometheus_stability,
            self.player_state.prometheus_trust,
        )?;
        Ok(())
    }

    fn complete_step(
        &mut self,
        level_num: usize,
        step_index: usize,
        step: &LevelStep,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            self.ui
                .show_step(
                    step,
                    step_index + 1,
                    self.level_manager
                        .as_ref()
                        .expect("level manager initialized")
                        .get_level(level_num)
                        .steps
                        .len(),
                    &self.player_state.lesson_depth,
                )?;

            let result = self.run_quiz(&step.quiz)?;
            self.player_state.add_quiz_score(result.score);

            if result.score >= PASS_PERCENT {
                self.player_state.apply_quiz_result(result.score, true);
                self.player_state.add_xp(result.xp_earned);
                self.ui.show_mastery_result(
                    step.title.as_str(),
                    result.score,
                    result.correct_answers,
                    result.total_questions,
                    result.xp_earned,
                    self.player_state.prometheus_stability,
                    self.player_state.prometheus_trust,
                )?;
                self.player_state.update_step_progress(level_num, step_index);

                if let Some(challenge) = &step.challenge {
                    self.complete_challenge(challenge)?;
                }

                self.save_state()?;
                return Ok(());
            }

            self.player_state.apply_quiz_result(result.score, false);
            self.ui.show_mastery_retry(
                step.title.as_str(),
                result.score,
                PASS_PERCENT,
                &result.missed_topics,
                self.player_state.prometheus_stability,
                self.player_state.prometheus_trust,
            )?;
        }
    }

    fn complete_challenge(
        &mut self,
        challenge: &Challenge,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.ui.show_challenge(challenge)?;
        if response.trim().chars().count() >= 20 {
            self.player_state.add_xp(challenge.xp_reward as u64);
            self.ui.show_message(&format!(
                "\nReflection captured. +{} XP for completing the challenge thoughtfully.",
                challenge.xp_reward
            ));
            self.ui.wait_for_enter()?;
        } else {
            self.ui.show_message(
                "\nChallenge noted, but no bonus XP was awarded because the response was too short.",
            );
            self.ui.wait_for_enter()?;
        }
        Ok(())
    }

    fn run_quiz(
        &self,
        questions: &[QuizQuestion],
    ) -> Result<QuizResult, Box<dyn std::error::Error>> {
        if questions.is_empty() {
            return Ok(QuizResult {
                score: 100,
                xp_earned: 0,
                correct_answers: 0,
                total_questions: 0,
                missed_topics: vec![],
            });
        }

        let mut correct = 0_u32;
        let mut xp_earned = 0_u64;
        let mut missed_topics = Vec::new();

        for question in questions {
            self.ui.ask_question(question)?;
            let answer = self.ui.get_input()?;

            if Self::is_correct_answer(answer.as_str(), question) {
                correct += 1;
                xp_earned += question.xp_reward as u64;
                self.ui.show_correct(question.explanation.as_str())?;
            } else {
                missed_topics.push(format!(
                    "{} -> {}",
                    question.question, question.explanation
                ));
                self.ui
                    .show_wrong(question.correct_answer.as_str(), question.explanation.as_str())?;
            }
        }

        let score = ((correct * 100) / questions.len() as u32).min(100);
        Ok(QuizResult {
            score,
            xp_earned,
            correct_answers: correct,
            total_questions: questions.len() as u32,
            missed_topics,
        })
    }

    fn is_correct_answer(answer: &str, question: &QuizQuestion) -> bool {
        let normalized = answer.trim().to_uppercase();
        if normalized == question.correct_answer {
            return true;
        }

        if let Some(index) = Self::answer_index(question.correct_answer.as_str()) {
            if let Some(option) = question.options.get(index) {
                return normalized == option.trim().to_uppercase();
            }
        }

        false
    }

    fn answer_index(answer: &str) -> Option<usize> {
        let first = answer.chars().next()?.to_ascii_uppercase();
        if ('A'..='Z').contains(&first) {
            Some((first as u8 - b'A') as usize)
        } else {
            None
        }
    }

    fn get_save_path() -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("neural_ascent");
        fs::create_dir_all(&path).ok();
        path.push("savegame.json");
        path
    }

    fn load_or_create_state() -> Result<PlayerState, Box<dyn std::error::Error>> {
        let path = Self::get_save_path();
        if path.exists() {
            let data = fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&data)?)
        } else {
            Ok(PlayerState::new())
        }
    }

    fn save_state(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(&self.player_state)?;
        fs::write(Self::get_save_path(), data)?;
        Ok(())
    }
}

struct QuizResult {
    score: u32,
    xp_earned: u64,
    correct_answers: u32,
    total_questions: u32,
    missed_topics: Vec<String>,
}

//! UI MODULE - PROMETHEUS
use crate::levels::{Challenge, LevelStep, QuizQuestion};
use crate::state::LessonDepth;
use crate::story::StorySegment;
use crossterm::terminal;
use std::io::{self, Write};

pub struct UI;

impl UI {
    pub fn new() -> Self {
        Self
    }

    pub fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1J\x1B[3J\x1B[H");
        io::stdout().flush().ok();
    }

    pub fn show_prologue(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_screen();
        let version = env!("CARGO_PKG_VERSION");
        let lines = vec![
            String::new(),
            "    ███╗   ██╗███████╗██╗   ██╗██████╗  █████╗ ██╗".to_string(),
            "    ████╗  ██║██╔════╝██║   ██║██╔══██╗██╔══██╗██║".to_string(),
            "    ██╔██╗ ██║█████╗  ██║   ██║██████╔╝███████║██║".to_string(),
            "    ██║╚██╗██║██╔══╝  ██║   ██║██╔══██╗██╔══██║██║".to_string(),
            "    ██║ ╚████║███████╗╚██████╔╝██║  ██║██║  ██║███████╗".to_string(),
            "    ╚═╝  ╚═══╝╚══════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝".to_string(),
            String::new(),
            "     █████╗ ███████╗ ██████╗███████╗███╗   ██╗████████╗".to_string(),
            "    ██╔══██╗██╔════╝██╔════╝██╔════╝████╗  ██║╚══██╔══╝".to_string(),
            "    ███████║███████╗██║     █████╗  ██╔██╗ ██║   ██║".to_string(),
            "    ██╔══██║╚════██║██║     ██╔══╝  ██║╚██╗██║   ██║".to_string(),
            "    ██║  ██║███████║╚██████╗███████╗██║ ╚████║   ██║".to_string(),
            "    ╚═╝  ╚═╝╚══════╝ ╚═════╝╚══════╝╚═╝  ╚═══╝   ╚═╝".to_string(),
            String::new(),
            format!("    BUILD VERSION {}", version),
            String::new(),
            "    LYRA RECOVERY LINK ESTABLISHED.".to_string(),
            "    PROMETHEUS remains locked behind instructional safeguards.".to_string(),
            "    You will unlock the system by understanding it, not by guessing.".to_string(),
            String::new(),
            "    This is a guided recovery simulation.".to_string(),
            "    Each lesson explains a real AI concept, shows why it matters, and".to_string(),
            "    requires you to demonstrate understanding before the next lock opens.".to_string(),
            String::new(),
            "    [ Press ENTER to continue ]".to_string(),
        ];
        self.render_fullscreen(lines)?;
        self.wait_for_enter()
    }

    pub fn show_stakes(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_screen();
        let lines = vec![
            String::new(),
            "    MISSION BRIEFING".to_string(),
            String::new(),
            "    PROMETHEUS was built as a high-capability AI system for research,".to_string(),
            "    retrieval, planning, and decision support across critical domains.".to_string(),
            "    Right now it is locked behind instructional safeguards because the".to_string(),
            "    system cannot be trusted by people who do not understand the stack.".to_string(),
            String::new(),
            "    LYRA will guide you through the recovery path:".to_string(),
            "    - choose Guided or Deep Dive mode".to_string(),
            "    - start from any lesson you want".to_string(),
            "    - study the concept and real-world examples".to_string(),
            "    - score at least 70% to unlock the next gate".to_string(),
            "    - revisit weak areas when the system sends you back".to_string(),
            String::new(),
            "    PROMETHEUS does not unlock through speed.".to_string(),
            "    It unlocks through demonstrated understanding.".to_string(),
            String::new(),
            "    [ Press ENTER to continue ]".to_string(),
        ];
        self.render_fullscreen(lines)?;
        self.wait_for_enter()
    }

    pub fn lesson_depth_menu(
        &self,
        current: &LessonDepth,
    ) -> Result<LessonDepth, Box<dyn std::error::Error>> {
        self.clear_screen();
        let current_label = match current {
            LessonDepth::Guided => "Guided",
            LessonDepth::DeepDive => "Deep Dive",
        };
        let lines = vec![
            String::new(),
            "    LEARNING MODE".to_string(),
            String::new(),
            "    [1] Guided".to_string(),
            "        Core explanation, examples, and quiz preparation.".to_string(),
            String::new(),
            "    [2] Deep Dive".to_string(),
            "        Adds prerequisite definitions, richer system reasoning, and".to_string(),
            "        more real-world framing before each quiz.".to_string(),
            String::new(),
            format!("    Current saved mode: {}", current_label),
            String::new(),
            "    Enter choice (1-2, ENTER keeps current):".to_string(),
        ];
        self.render_fullscreen(lines)?;
        io::stdout().flush()?;
        let input = self.get_input()?;
        let choice = match input.trim() {
            "1" => LessonDepth::Guided,
            "2" => LessonDepth::DeepDive,
            _ => current.clone(),
        };
        Ok(choice)
    }

    pub fn level_select_menu(
        &self,
        current_level: usize,
        levels: &[(usize, String)],
    ) -> Result<usize, Box<dyn std::error::Error>> {
        self.clear_screen();
        let mut lines = vec![
            String::new(),
            "    START LEVEL".to_string(),
            String::new(),
            format!(
                "    Enter a level number from 1-{}.",
                levels.len()
            ),
            format!(
                "    Press ENTER to resume from level {}.",
                current_level + 1
            ),
            String::new(),
            "    Available lessons:".to_string(),
        ];

        for (idx, title) in levels {
            lines.push(format!("    {:>2}. {}", idx + 1, title));
        }

        lines.push(String::new());
        lines.push("    Start from level:".to_string());
        self.render_fullscreen(lines)?;
        io::stdout().flush()?;
        let input = self.get_input()?;
        if input.trim().is_empty() {
            return Ok(current_level);
        }
        let chosen = input
            .trim()
            .parse::<usize>()
            .ok()
            .and_then(|n| n.checked_sub(1))
            .unwrap_or(current_level);
        Ok(chosen.min(levels.len().saturating_sub(1)))
    }

    pub fn show_level_intro(
        &self,
        level: usize,
        total_levels: usize,
        name: &str,
        title: &str,
        overview: &str,
        prerequisites: &[String],
        story: &StorySegment,
        stability: i32,
        trust: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_screen();
        let progress = ((level as f32 / total_levels as f32) * 20.0).round() as usize;
        let bar = "█".repeat(progress.min(20)) + &"░".repeat(20usize.saturating_sub(progress));

        println!(
            "
╔══════════════════════════════════════════════════════════════════════╗
║ LEVEL {:>2}/{:<2} [{}]                                              ║
╚══════════════════════════════════════════════════════════════════════╝",
            level, total_levels, bar
        );
        println!("\n    Learner: {}", name);
        println!("    Topic: {}", title);
        println!("    Overview: {}", overview);
        println!("    Arc: {}", story.act);
        println!("    Setting: {}", story.setting);
        println!("    PROMETHEUS stability: {}%", stability);
        println!("    PROMETHEUS trust: {}%", trust);
        if !prerequisites.is_empty() {
            println!("    Prerequisites: {}", prerequisites.join(", "));
        }
        println!("\n    Mission Briefing");
        self.print_wrapped(&story.narrative, 74, "    ");
        for dialogue in &story.character_dialogue {
            println!("\n    {}: {}", dialogue.speaker, dialogue.text);
        }
        println!("\n    [ Press ENTER to open the lesson ]");
        self.wait_for_enter()
    }

    pub fn show_step(
        &self,
        step: &LevelStep,
        step_number: usize,
        total_steps: usize,
        lesson_depth: &LessonDepth,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_screen();
        let mode = if step.is_synthesis {
            "SYNTHESIS CHECKPOINT"
        } else {
            "LESSON"
        };
        println!(
            "
╔══════════════════════════════════════════════════════════════════════╗
║ {} {:>2}/{:<2}: {:<37}║
╚══════════════════════════════════════════════════════════════════════╝",
            mode,
            step_number,
            total_steps,
            Self::fit_title(step.title.as_str(), 37)
        );

        println!(
            "\n    Mode: {}",
            match lesson_depth {
                LessonDepth::Guided => "Guided",
                LessonDepth::DeepDive => "Deep Dive",
            }
        );

        println!("\n    Foundations");
        self.print_wrapped(&step.foundations, 74, "    ");

        println!("\n    Concept");
        self.print_wrapped(&step.theory, 74, "    ");

        if !step.code_examples.is_empty() {
            println!("\n    Code Examples");
            for example in &step.code_examples {
                println!("\n    [{}] {}", example.language, example.title);
                self.print_wrapped(&example.explanation, 74, "      ");
                for line in example.code.lines() {
                    println!("      {}", line);
                }
            }
        }

        if !step.real_world_examples.is_empty() {
            println!("\n    Why It Matters");
            for example in &step.real_world_examples {
                println!("\n    - {}", example.title);
                self.print_wrapped(&example.description, 72, "      ");
                self.print_wrapped(&format!("Impact: {}", example.impact), 72, "      ");
            }
        }

        if step.is_synthesis {
            println!("\n    This checkpoint mixes earlier concepts. Expect system-level reasoning.");
        }

        println!("\n    [ Press ENTER when you are ready for the quiz ]");
        self.wait_for_enter()
    }

    pub fn ask_question(&self, q: &QuizQuestion) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_screen();
        println!(
            "
╔══════════════════════════════════════════════════════════════════════╗
║                              QUIZ                                   ║
╚══════════════════════════════════════════════════════════════════════╝"
        );
        self.print_wrapped(&q.question, 74, "\n    ");
        println!("\n");
        for (i, option) in q.options.iter().enumerate() {
            println!("    {}. {}", (b'A' + i as u8) as char, option);
        }
        print!("\n    Answer: ");
        io::stdout().flush()?;
        Ok(())
    }

    pub fn show_correct(&self, explanation: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_screen();
        println!(
            "
╔══════════════════════════════════════════════════════════════════════╗
║                             CORRECT                                 ║
╚══════════════════════════════════════════════════════════════════════╝"
        );
        self.print_wrapped(explanation, 74, "\n    ");
        println!("\n\n    [ Press ENTER to continue ]");
        self.wait_for_enter()
    }

    pub fn show_wrong(
        &self,
        correct_answer: &str,
        explanation: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_screen();
        println!(
            "
╔══════════════════════════════════════════════════════════════════════╗
║                            NOT YET                                  ║
╚══════════════════════════════════════════════════════════════════════╝"
        );
        println!("\n    Correct answer: {}", correct_answer);
        self.print_wrapped(explanation, 74, "\n    ");
        println!("\n\n    [ Press ENTER to continue ]");
        self.wait_for_enter()
    }

    pub fn show_mastery_retry(
        &self,
        title: &str,
        score: u32,
        required: u32,
        missed_topics: &[String],
        stability: i32,
        trust: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_screen();
        println!(
            "
╔══════════════════════════════════════════════════════════════════════╗
║                         REVIEW REQUIRED                             ║
╚══════════════════════════════════════════════════════════════════════╝

    Lesson: {}
    Score: {}%
    Required: {}%
    PROMETHEUS stability: {}%
    PROMETHEUS trust: {}%
",
            title, score, required, stability, trust
        );

        if !missed_topics.is_empty() {
            println!("    Review these ideas before retrying:\n");
            for topic in missed_topics {
                self.print_wrapped(&format!("- {}", topic), 72, "    ");
            }
        }

        println!("\n    [ Press ENTER to revisit the lesson ]");
        self.wait_for_enter()
    }

    pub fn show_mastery_result(
        &self,
        title: &str,
        score: u32,
        correct_answers: u32,
        total_questions: u32,
        xp_earned: u64,
        stability: i32,
        trust: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_screen();
        let response = if score == 100 {
            "PROMETHEUS stabilizes for a moment. 'Precision. No guessing. Continue.'"
        } else if score >= 85 {
            "PROMETHEUS inclines its head. 'Strong work. You understood the mechanism, not just the vocabulary.'"
        } else {
            "PROMETHEUS steps aside from the gate. 'You passed, but keep the weak points in memory. They return later.'"
        };

        println!(
            "
╔══════════════════════════════════════════════════════════════════════╗
║                         MASTERY CONFIRMED                           ║
╚══════════════════════════════════════════════════════════════════════╝

    Lesson: {}
    Score: {}%
    Correct: {}/{}
    XP earned from quiz: {}
    PROMETHEUS stability: {}%
    PROMETHEUS trust: {}%
",
            title, score, correct_answers, total_questions, xp_earned, stability, trust
        );
        self.print_wrapped(response, 74, "    ");
        println!("\n\n    [ Press ENTER to continue ]");
        self.wait_for_enter()
    }

    pub fn show_challenge(
        &self,
        challenge: &Challenge,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.clear_screen();
        println!(
            "
╔══════════════════════════════════════════════════════════════════════╗
║                         REFLECTION CHALLENGE                        ║
╚══════════════════════════════════════════════════════════════════════╝

    {}
",
            challenge.title
        );
        self.print_wrapped(&challenge.description, 74, "    ");
        print!("\n\n    Your response: ");
        io::stdout().flush()?;
        self.get_input()
    }

    pub fn level_complete(
        &self,
        level: usize,
        bonus: u64,
        total_xp: u64,
        stability: i32,
        trust: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_screen();
        let narrative = match level {
            1..=5 => "The first memory locks disengage. PROMETHEUS no longer treats you as an intruder.",
            6..=10 => "The simulation deepens. You are no longer learning definitions; you are seeing the machinery.",
            11..=16 => "PROMETHEUS stops posturing and starts revealing operational truth: context, tools, and the cost of power.",
            17..=21 => "The world pushes back. Capability now collides with production, abuse, and harm.",
            _ => "The final systems come online. Understanding is becoming authority.",
        };
        let lines = vec![
            String::new(),
            "    LEVEL COMPLETE".to_string(),
            String::new(),
            format!("    Level cleared: {}", level),
            format!("    Completion bonus: {} XP", bonus),
            format!("    Total XP: {}", total_xp),
            format!("    PROMETHEUS stability: {}%", stability),
            format!("    PROMETHEUS trust: {}%", trust),
            String::new(),
            format!("    {}", narrative),
            String::new(),
            "    [ Press ENTER for the next level ]".to_string(),
        ];
        self.render_fullscreen(lines)?;
        self.wait_for_enter()
    }

    pub fn show_victory(
        &self,
        name: &str,
        xp: u64,
        stability: i32,
        trust: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_screen();
        let lines = vec![
            String::new(),
            "    PROMETHEUS RECOVERY COMPLETE".to_string(),
            String::new(),
            format!(
                "    {}, you completed the ascent by demonstrating mastery instead",
                name
            ),
            "    of clicking through the simulation.".to_string(),
            String::new(),
            format!("    Final XP: {}", xp),
            format!("    Final stability: {}%", stability),
            format!("    Final trust: {}%", trust),
            String::new(),
            "    LYRA: Do not just use AI. Interrogate it, evaluate it, constrain".to_string(),
            "    it, and build with a clear model of what it can and cannot do.".to_string(),
            String::new(),
            "    [ End of simulation ]".to_string(),
        ];
        self.render_fullscreen(lines)?;
        Ok(())
    }

    pub fn wait_for_enter(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(())
    }

    pub fn get_input(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    pub fn show_message(&self, msg: &str) {
        println!("{}", msg);
    }

    fn print_wrapped(&self, text: &str, width: usize, prefix: &str) {
        for paragraph in text.split('\n') {
            let mut line = String::new();
            for word in paragraph.split_whitespace() {
                if !line.is_empty() && line.len() + 1 + word.len() > width {
                    println!("{}{}", prefix, line);
                    line.clear();
                }
                if !line.is_empty() {
                    line.push(' ');
                }
                line.push_str(word);
            }
            if !line.is_empty() {
                println!("{}{}", prefix, line);
            } else {
                println!();
            }
        }
    }

    fn fit_title(title: &str, width: usize) -> String {
        let mut trimmed = title.chars().take(width).collect::<String>();
        let len = trimmed.chars().count();
        if len < width {
            trimmed.push_str(&" ".repeat(width - len));
        }
        trimmed
    }

    fn render_fullscreen(&self, lines: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let (_, height) = terminal::size().unwrap_or((80, 30));
        let total_lines = lines.len() as u16;
        let top_padding = height.saturating_sub(total_lines) / 3;

        for _ in 0..top_padding {
            println!();
        }
        for line in &lines {
            println!("{}", line);
        }
        let used = top_padding + total_lines;
        for _ in used..height {
            println!();
        }
        io::stdout().flush()?;
        Ok(())
    }
}

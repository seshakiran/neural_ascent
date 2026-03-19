//! UI MODULE - PROMETHEUS
use crate::levels::{Challenge, LevelStep, QuizQuestion};
use crate::state::LessonDepth;
use crate::story::StorySegment;
use std::io::{self, Write};

pub struct UI;

impl UI {
    pub fn new() -> Self {
        Self
    }

    pub fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1J\x1B[H");
        io::stdout().flush().ok();
    }

    pub fn show_prologue(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_screen();
        println!(
            "
╔══════════════════════════════════════════════════════════════════════╗
║                           NEURAL ASCENT                             ║
╚══════════════════════════════════════════════════════════════════════╝

    YEAR 2031.

    PROMETHEUS, the world's most ambitious AI system, has gone silent.
    Nexus Labs cannot recover it. The public only sees outages. Inside the
    lab, you find a hidden terminal and a direct challenge:

    'If you want to wake me up, do not guess. Understand.'

    This is not a trivia sprint. Each level teaches a concept, tests whether
    you actually grasp it, and sends you back to review if you do not.

    [ Press ENTER to continue ]"
        );
        self.wait_for_enter()
    }

    pub fn show_stakes(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.clear_screen();
        println!(
            "
╔══════════════════════════════════════════════════════════════════════╗
║                           THE TERMS                                 ║
╚══════════════════════════════════════════════════════════════════════╝

    PROMETHEUS will not unlock on random clicks.

    You must:
    - read the lesson
    - choose Guided or Deep Dive mode
    - study the examples
    - score at least 70% on the quiz
    - revisit the material if you miss core ideas

    Bonus challenges now reward thoughtful written reflections instead of
    free XP for typing anything.

    [ Press ENTER to begin ]"
        );
        self.wait_for_enter()
    }

    pub fn lesson_depth_menu(
        &self,
        current: &LessonDepth,
    ) -> Result<LessonDepth, Box<dyn std::error::Error>> {
        self.clear_screen();
        println!(
            "
╔══════════════════════════════════════════════════════════════════════╗
║                         LEARNING MODE                               ║
╚══════════════════════════════════════════════════════════════════════╝

    Choose how PROMETHEUS should teach:

    [1] Guided
        Shorter explanations with the core concept, examples, and quiz prep.

    [2] Deep Dive
        Adds more definitions, system reasoning, and real-world framing before
        each quiz.

    Current saved mode: {}

    Enter choice (1-2, ENTER keeps current): ",
            match current {
                LessonDepth::Guided => "Guided",
                LessonDepth::DeepDive => "Deep Dive",
            }
        );
        io::stdout().flush()?;
        let input = self.get_input()?;
        let choice = match input.trim() {
            "1" => LessonDepth::Guided,
            "2" => LessonDepth::DeepDive,
            _ => current.clone(),
        };
        Ok(choice)
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
        println!("\n    Story");
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
        println!(
            "
╔══════════════════════════════════════════════════════════════════════╗
║                         LEVEL COMPLETE                              ║
╚══════════════════════════════════════════════════════════════════════╝

    Level cleared: {}
    Completion bonus: {} XP
    Total XP: {}
    PROMETHEUS stability: {}%
    PROMETHEUS trust: {}%

    [ Press ENTER for the next level ]",
            level, bonus, total_xp, stability, trust
        );
        let narrative = match level {
            1..=5 => "The first memory locks disengage. PROMETHEUS no longer treats you as an intruder.",
            6..=10 => "The simulation deepens. You are no longer learning definitions; you are seeing the machinery.",
            11..=16 => "PROMETHEUS stops posturing and starts revealing operational truth: context, tools, and the cost of power.",
            17..=21 => "The world pushes back. Capability now collides with production, abuse, and harm.",
            _ => "The final systems come online. Understanding is becoming authority.",
        };
        self.print_wrapped(narrative, 74, "\n    ");
        println!();
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
        println!(
            "
╔══════════════════════════════════════════════════════════════════════╗
║                           PROMETHEUS AWAKENS                        ║
╚══════════════════════════════════════════════════════════════════════╝

    {}, you completed the ascent by demonstrating mastery instead of
    clicking through it.

    Final XP: {}
    Final stability: {}%
    Final trust: {}%

    PROMETHEUS returns one final message:
    'Do not just use AI. Interrogate it, evaluate it, constrain it,
    and build with a clear model of what it can and cannot do.'

    [ End of simulation ]",
            name, xp, stability, trust
        );
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
}

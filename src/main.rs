//! ======================================================================================
//! NEURAL ASCENT - The Ultimate AI Mastery Quest
//! ======================================================================================
//! A story-driven TUI game that takes you from AI novice to expert
//!
//! This game covers EVERYTHING:
//! - AI Fundamentals & History
//! - Machine Learning Basics (Supervised, Unsupervised, Reinforcement)
//! - Neural Networks Deep Dive
//! - Deep Learning Architectures
//! - Transformers & Attention Mechanisms
//! - Large Language Models (LLMs)
//! - Prompt Engineering Mastery
//! - Generative AI (Images, Audio, Video)
//! - Diffusion Models
//! - AI System Design
//! - AI Security & Threats
//! - AI Governance & Ethics
//! - Real-world Interview Questions
//! - Code Samples & Practical Examples
//! - And MUCH more...
//!
//! Author: Neural Ascent Team
//! Version: 1.0.0
//! ======================================================================================

mod game;
mod levels;
mod state;
mod story;
mod ui;

use std::panic;
use std::process;

fn main() {
    // Set up panic handler for graceful error handling
    panic::set_hook(Box::new(|panic_info| {
        eprintln!("\n⚠️  GAME PANIC: {}", panic_info);
        eprintln!("The Neural Network has encountered an unexpected error!");
        eprintln!("Please restart your journey through the ascent...\n");
        process::exit(1);
    }));

    // Launch the game
    let mut game = match game::NeuralAscent::new() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("\n❌ Failed to initialize game: {}", e);
            process::exit(1);
        }
    };
    
    if let Err(e) = game.run() {
        eprintln!("\n❌ Game Error: {}", e);
        process::exit(1);
    }
}

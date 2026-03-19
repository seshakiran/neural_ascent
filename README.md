# NEURAL ASCENT - The Ultimate AI Mastery Quest

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.77+-dea584?style=for-the-badge&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/TUI-Terminal-yellow?style=for-the-badge" alt="TUI">
  <img src="https://img.shields.io/badge/AI-Learning-blue?style=for-the-badge" alt="AI Learning">
</p>

## 🎮 The Ultimate AI Learning Game

**NEURAL ASCENT** is a story-driven Terminal User Interface (TUI) game that takes you on an epic journey from AI novice to expert. With **24 immersive levels**, interactive quizzes, real-world code examples, and adaptive difficulty, you'll master AI concepts like never before.

## ✨ Features

### 📚 Comprehensive Curriculum (24 Levels)
- **Level 1-2**: AI Fundamentals & History, Python/Math Basics
- **Level 3-4**: Machine Learning (Supervised, Unsupervised)
- **Level 5-6**: Neural Networks & Deep Learning
- **Level 7-8**: Transformers & Attention Mechanisms
- **Level 9-10**: Large Language Models (LLMs) & Prompt Engineering
- **Level 11-12**: RAG & Fine-tuning LLMs
- **Level 13**: AI Agents - Introduction
- **Level 14**: Agentic Patterns (ReAct, Tool Use, Planning)
- **Level 15**: Code Agents (Devin, Multi-Agent Systems)
- **Level 16**: Context Engineering (Memory, RAG, System Prompts)
- **Level 17**: Training at Scale
- **Level 18-19**: Interview Preparation
- **Level 20-22**: AI Security, Ethics & Governance
- **Level 23-24**: The Final Ascension

### 🎯 Game Mechanics
- **Interactive Quizzes** with XP rewards
- **Code Examples** in Python
- **Real-world Examples** from industry
- **Challenges** for bonus XP
- **Adaptive Difficulty** based on performance
- **Progress Saving** - resume anytime

### 📖 What's Covered
- AI Fundamentals & History
- Machine Learning (Supervised, Unsupervised, Reinforcement)
- Neural Networks Deep Dive
- CNNs & RNNs
- Transformers & Attention
- Large Language Models (GPT, BERT)
- Prompt Engineering (Zero-shot, Few-shot, Chain-of-thought)
- **AI Agents** (Agent loops, autonomous systems)
- **Agentic Patterns** (ReAct, Tool Use, Planning)
- **Code Agents** (Devin, Cursor, Multi-Agent systems)
- **Context Engineering** (Memory systems, RAG, System prompts)
- Generative AI (Images, Audio, Video)
- Diffusion Models
- RAG & AI Agents
- Fine-tuning Techniques (LoRA, RLHF)
- Training at Scale (distributed training, mixed precision)
- System Design for ML
- AI Security (adversarial attacks, prompt injection)
- AI Ethics & Bias
- AI Governance (EU AI Act, regulations)
- Interview Questions & Answers
- Real-world Code Samples

### 🏆 Achievement System
- Novice Learner → AI Apprentice → ML Practitioner → Deep Learning Adept
- Neural Network Expert → Transformer Master → LLM Wizard → AI Grandmaster → AI Sage

## 🚀 Getting Started

### Prerequisites
- Rust (1.77+)
- Cargo

### Installation

```bash
# Clone the repository
cd ~/Downloads/Projects

# Build the game
cd neural_ascent
cargo build --release

# Run the game
./target/release/neural_ascent
```

### Quick Start

```bash
# Just run!
cargo run --release
```

## 🎮 How to Play

1. **Start the game** - Press ENTER on the title screen
2. **Read the theory** - Each level has comprehensive explanations
3. **Study the code** - Real Python examples for each concept
4. **Answer quizzes** - Test your knowledge and earn XP
5. **Complete challenges** - Bonus XP for extra practice
6. **Progress through levels** - Each level builds on the last
7. **Master AI** - Complete all 20 levels to become an AI Master!

## 📁 Project Structure

```
neural_ascent/
├── data/
│   ├── curriculum_seed.json   # Editable curriculum source
│   └── curriculum.sqlite      # Runtime SQLite lesson database
├── docs/
│   └── curriculum_workflow.md # How to edit/sync/validate curriculum
├── src/
│   ├── main.rs        # Entry point
│   ├── game.rs        # Game engine & logic
│   ├── levels.rs      # SQLite curriculum loader
│   ├── state.rs       # Player state management
│   ├── story.rs      # Narrative system
│   └── ui.rs         # Terminal UI rendering
├── Cargo.toml        # Dependencies
└── README.md        # This file
```

## ✍️ Curriculum Editing

Curriculum content is authored in [data/curriculum_seed.json](/Users/itsthematrix/Downloads/Projects/neural_ascent/data/curriculum_seed.json) and loaded at runtime from [data/curriculum.sqlite](/Users/itsthematrix/Downloads/Projects/neural_ascent/data/curriculum.sqlite). The editing workflow is documented in [docs/curriculum_workflow.md](/Users/itsthematrix/Downloads/Projects/neural_ascent/docs/curriculum_workflow.md).

## 🔧 Technical Details

- **Language**: Rust (blazingly fast!)
- **UI Framework**: Custom TUI with ANSI colors
- **State Persistence**: JSON save files
- **Adaptive Difficulty**: Tracks quiz performance
- **Save Location**: `~/.local/share/neural_ascent/savegame.json`

## 📝 Why Rust?

We chose Rust for this project because:
- **Blazingly Fast** - Native performance
- **Memory Safe** - No crashes or data races
- **Cross-Platform** - Works on Mac, Linux, Windows
- **Small Binary** - ~550KB release build
- **Fun to Write** - Modern language features

## 🤝 Contributing

This is an open project! Feel free to:
- Add more levels
- Improve the UI
- Add new quiz questions
- Fix bugs
- Enhance the story

## 📚 Learning Resources

The game covers concepts from many sources:
- DeepLearning.AI courses
- Stanford CS229/CS231N
- "Hands-On Machine Learning" by Aurélien Géron
- "The Deep Learning Book" by Goodfellow et al.
- Research papers (Attention Is All You Need, etc.)

## ⚠️ Note

This game is designed for educational purposes. While it covers advanced topics, real AI mastery requires:
- Building actual projects
- Reading research papers
- Staying updated with the field
- Practice, practice, practice!

## 🎉 The Journey Awaits

> "The future belongs to those who understand AI."
> — Your friend (who may or may not exist)

Are you ready to ascend? 🚀

---

**Created with ❤️ and lots of ☕**

Version: 1.0.0
License: MIT

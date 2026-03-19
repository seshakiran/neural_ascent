//! STORY ENGINE - Guide-driven narrative

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub speaker: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorySegment {
    pub level_id: usize,
    pub title: String,
    pub act: String,
    pub setting: String,
    pub narrative: String,
    pub character_dialogue: Vec<Dialogue>,
}

impl StorySegment {
    pub fn render_for_player(&self, player_name: &str) -> Self {
        let mut rendered = self.clone();
        rendered.title = rendered.title.replace("{player_name}", player_name);
        rendered.act = rendered.act.replace("{player_name}", player_name);
        rendered.setting = rendered.setting.replace("{player_name}", player_name);
        rendered.narrative = rendered.narrative.replace("{player_name}", player_name);
        rendered.character_dialogue = rendered
            .character_dialogue
            .iter()
            .map(|dialogue| Dialogue {
                speaker: dialogue.speaker.clone(),
                text: dialogue.text.replace("{player_name}", player_name),
            })
            .collect();
        rendered
    }
}

pub struct StoryEngine {
    story_segments: Vec<StorySegment>,
}

impl StoryEngine {
    pub fn new() -> Self {
        Self {
            story_segments: Self::create_story(),
        }
    }

    pub fn get_level_story(&self, level_id: usize) -> &StorySegment {
        self.story_segments
            .get(level_id)
            .unwrap_or(&self.story_segments[0])
    }

    fn create_story() -> Vec<StorySegment> {
        vec![
            story(
                0,
                "Wake Sequence",
                "Act I: Briefing",
                "Nexus Labs, Recovery Terminal",
                "Emergency lighting washes the lab in blue-white pulses as the recovery terminal comes online. You are {player_name}, and this simulation exists for one reason: to help you unlock PROMETHEUS by understanding the ideas it was built on. I am LYRA, your guide through the recovery sequence. I will explain what each lesson means, why it matters in real systems, and what you need to prove before the next lock opens.",
                vec![
                    dialogue("LYRA", "You are not wandering through a dreamscape. You are in a guided recovery simulation built from PROMETHEUS's training archive."),
                    dialogue("LYRA", "Each lesson restores part of the system because understanding the stack is how you regain control of it."),
                ],
            ),
            story(
                1,
                "What Intelligence Means",
                "Act I: Briefing",
                "Training Archive, Orientation Deck",
                "We start with the foundation. Before you can reason clearly about AI, you need a clean definition of intelligence. In this level, focus on the difference between raw capability and adaptable goal-directed behavior. If you mix up speed, fluency, or memorization with intelligence here, every later lesson will sound impressive without really being clear.",
                vec![
                    dialogue("LYRA", "I will guide you level by level. PROMETHEUS is the system we are trying to understand, stabilize, and eventually trust again."),
                    dialogue("LYRA", "Start here: intelligence is about achieving goals under changing conditions, not merely producing fast output."),
                ],
            ),
            story(
                2,
                "Rules Before Learning",
                "Act I: Briefing",
                "Archive Wing, Symbolic Systems",
                "This section covers symbolic AI, the older style of building intelligence with explicit rules, logic, and search. I want you to notice two things at the same time: where this approach is still strong, and where it breaks down once the world becomes noisy, ambiguous, or too large to describe by hand.",
                vec![
                    dialogue("LYRA", "If a domain is governed by explicit procedures, symbolic methods can still be the most reliable tool in the room."),
                    dialogue("LYRA", "Your job is to notice both the strength and the boundary."),
                ],
            ),
            story(
                3,
                "Learning From Data",
                "Act I: Briefing",
                "Archive Wing, Learning Systems",
                "Now we shift from hand-written rules to learned patterns. In machine learning, you let a model fit parameters from examples instead of specifying every behavior yourself. The tradeoff is important: you gain scale and flexibility, but you inherit responsibility for the data, the objective, and the evaluation setup.",
                vec![
                    dialogue("LYRA", "When we say a model learns, we mean it adjusts parameters to do better on a defined objective."),
                    dialogue("LYRA", "The important question is whether that objective matches the behavior you actually want."),
                ],
            ),
            story(
                4,
                "Supervision and Targets",
                "Act I: Briefing",
                "Archive Wing, Label Forge",
                "In supervised learning, human judgment gets turned into targets. Spam labels, prices, diagnoses, risk scores: these are the signals the model learns to reproduce. The key idea for this level is that labels feel objective when you first meet them, but they are design choices, and design choices always carry assumptions.",
                vec![
                    dialogue("LYRA", "A label is not reality. It is an operational decision about what the model should optimize."),
                    dialogue("LYRA", "That is why poorly chosen labels produce confident but unhelpful systems."),
                ],
            ),
            story(
                5,
                "Structure Without Labels",
                "Act I: Briefing",
                "Archive Wing, Unsupervised Lab",
                "Here the answer key disappears. This lesson is about unsupervised learning: finding patterns, clusters, and compressed structure when nobody labeled the data for you. The important question is not just whether you can discover structure. It is whether the structure helps with a real task afterward.",
                vec![
                    dialogue("LYRA", "No labels means no obvious target. You have to ask whether the pattern helps with retrieval, segmentation, anomaly detection, or some later decision."),
                    dialogue("LYRA", "That is why evaluation becomes less direct here."),
                ],
            ),
            story(
                6,
                "Why Depth Matters",
                "Act II: Models",
                "Representation Lab",
                "Now we get to neural networks. Treat them as layered systems that learn internal representations useful for the task. I want to demystify depth here. The point is not to admire large models. The point is to understand how successive transformations can build from simple signals toward abstractions that actually help with prediction or decision-making.",
                vec![
                    dialogue("LYRA", "A deeper network is not automatically smarter. It is simply capable of representing more complex transformations."),
                    dialogue("LYRA", "The real question is whether those transformations are aligned with the job you asked it to do."),
                ],
            ),
            story(
                7,
                "Optimization",
                "Act II: Models",
                "Optimization Core",
                "Do not rush this section. Training is optimization. A loss function defines what counts as being wrong, gradients describe how parameters affect that error, and the optimizer updates the model accordingly. The lesson is simple but critical: systems optimize the math you specify, not the intention you hoped the math would capture.",
                vec![
                    dialogue("LYRA", "If a model behaves badly, ask what the objective rewarded before blaming the architecture."),
                    dialogue("LYRA", "A loss function is not just a formula. It is a statement of what the system is allowed to care about."),
                ],
            ),
            story(
                8,
                "Architectures",
                "Act II: Models",
                "Architecture Gallery",
                "This level compares CNNs, RNNs, and transformers as different bets about structure. Architecture is not style. It is an inductive bias about what kinds of patterns the model should find easiest to learn, so the right question is always: what structure does this task actually have?",
                vec![
                    dialogue("LYRA", "Choose architecture by asking what structure the data really has: locality, sequence, flexible long-range context, or something else."),
                    dialogue("LYRA", "When the built-in assumptions match the task, learning becomes easier and more stable."),
                ],
            ),
            story(
                9,
                "Attention",
                "Act II: Models",
                "Transformer Hall",
                "This level is about attention: how a model decides which parts of the input matter to each other. What changed with transformers was not magic. It was a much better mechanism for connecting distant but relevant context, which is why long documents, code, and references became easier to model well.",
                vec![
                    dialogue("LYRA", "Do not think of attention as consciousness. Think of it as dynamic relevance weighting inside the model."),
                    dialogue("LYRA", "It changed the field because long-range dependency stopped being such a weak point."),
                ],
            ),
            story(
                10,
                "Embeddings",
                "Act II: Models",
                "Vector Space Lab",
                "Here we reframe language and meaning as geometry. Embeddings are vector representations that make similarity, retrieval, and ranking possible. I do not want you to just memorize the term. I want you to see why search systems, recommenders, and RAG pipelines depend on these spaces.",
                vec![
                    dialogue("LYRA", "An embedding is a representation, not a magical truth object."),
                    dialogue("LYRA", "If the representation is poor, the retrieval or ranking built on top of it will also be poor."),
                ],
            ),
            story(
                11,
                "Large Language Models",
                "Act III: Language Systems",
                "Autoregressive Chamber",
                "Here is the mental model I want you to keep: an LLM is a system trained to predict the next token from context at massive scale. That objective produces broad capabilities, which is why these models can feel so flexible. It also creates risk, because plausible continuation is not the same thing as truth or evidence.",
                vec![
                    dialogue("LYRA", "When an LLM sounds certain, that may reflect a strong token distribution, not strong evidence."),
                    dialogue("LYRA", "That difference is why grounding and evaluation matter so much."),
                ],
            ),
            story(
                12,
                "Prompting",
                "Act III: Language Systems",
                "Instruction Deck",
                "Treat prompting as interface design. In this level, focus on how task framing, examples, schema constraints, and evidence boundaries alter model behavior. Prompts are not magic spells. They are a way of shaping context, and poor context usually produces poor behavior.",
                vec![
                    dialogue("LYRA", "A good prompt reduces ambiguity. It does not create missing capabilities out of thin air."),
                    dialogue("LYRA", "Use prompts to shape behavior, then use evaluation to prove the behavior is real."),
                ],
            ),
            story(
                13,
                "Retrieval and Grounding",
                "Act III: Language Systems",
                "External Memory Array",
                "This level is where we stop pretending the model should answer everything from memory alone. Retrieval-augmented generation works by searching external sources, bringing relevant evidence into context, generating from that evidence, and making the support visible to the user.",
                vec![
                    dialogue("LYRA", "Grounding is not a patch for weakness. It is often the correct architecture for trustworthy answers."),
                    dialogue("LYRA", "If retrieval is wrong, the answer can still sound polished while being fundamentally unsupported."),
                ],
            ),
            story(
                14,
                "Fine-Tuning",
                "Act III: Language Systems",
                "Adaptation Lab",
                "This level is about when changing the model itself is justified. Fine-tuning is powerful because it changes persistent behavior, but that is exactly why it is risky. Your goal here is to distinguish what should be handled through prompts or retrieval from what truly needs to be pushed into the model weights.",
                vec![
                    dialogue("LYRA", "Prompting changes context. Fine-tuning changes the model."),
                    dialogue("LYRA", "Only pay the cost of weight changes when you truly need persistent specialization."),
                ],
            ),
            story(
                15,
                "Agents",
                "Act III: Language Systems",
                "Tool Control Deck",
                "Now we move from answering to acting. Agents have control flow, memory, tools, and side effects. That makes them useful, but it also makes them risky. Once a model can act through tools, bad assumptions stop being just reasoning mistakes and become operational problems.",
                vec![
                    dialogue("LYRA", "A chatbot replies. An agent plans, uses tools, checks results, and may change the world."),
                    dialogue("LYRA", "That is why tool boundaries and verification loops are part of the lesson, not optional safety extras."),
                ],
            ),
            story(
                16,
                "Context Engineering",
                "Act III: Language Systems",
                "Memory Assembly Deck",
                "Many teams learn this too late: model quality depends heavily on what information the model receives, in what order, and in what structure. This level is about memory, context windows, summaries, retrieval payloads, and the discipline of giving the model the right information instead of all information.",
                vec![
                    dialogue("LYRA", "More context is not the same thing as better context."),
                    dialogue("LYRA", "Many model failures are really information-assembly failures."),
                ],
            ),
            story(
                17,
                "Multimodal AI",
                "Act IV: Real Systems",
                "Cross-Modal Lab",
                "Now we move beyond text. This lesson is about what changes when models must align text, image, audio, and video. The opportunity is richer reasoning. The cost is harder evaluation, because each modality introduces its own structure, noise, privacy concerns, and failure modes.",
                vec![
                    dialogue("LYRA", "Multimodal systems widen capability, but they also widen the number of ways a system can be wrong."),
                    dialogue("LYRA", "You need stronger evidence standards, not looser ones."),
                ],
            ),
            story(
                18,
                "Generative Media",
                "Act IV: Real Systems",
                "Generation Studio",
                "This level treats image and video generation as modeling problems, not magic tricks. The main lesson is that realism is easy to over-trust. A system can look convincing while still failing on consistency, physics, provenance, or rights. I want you to become skeptical in a useful way.",
                vec![
                    dialogue("LYRA", "A good-looking frame is not the same thing as a coherent or trustworthy generation."),
                    dialogue("LYRA", "Always ask what was preserved across time, what evidence exists, and what risks were introduced."),
                ],
            ),
            story(
                19,
                "MLOps",
                "Act IV: Real Systems",
                "Operations Floor",
                "Now we turn the lesson into operations. Models do not stay good simply because they were once good. This level teaches deployment, monitoring, rollback, and drift so you understand that production AI is an ongoing system, not a frozen artifact.",
                vec![
                    dialogue("LYRA", "A launch is the beginning of model accountability, not the end of model work."),
                    dialogue("LYRA", "If you cannot observe degradation, you cannot claim the system is under control."),
                ],
            ),
            story(
                20,
                "Security",
                "Act IV: Real Systems",
                "Red Team Sandbox",
                "This section is direct for a reason. AI systems can be attacked through data, prompts, retrieval, and tools. The lesson is about adversarial thinking: what happens when inputs are malicious, permissions are loose, or external text is treated as trustworthy when it should not be.",
                vec![
                    dialogue("LYRA", "Prompt injection is dangerous because the model treats text as potentially actionable instruction."),
                    dialogue("LYRA", "Once tools are involved, software security and model behavior become inseparable."),
                ],
            ),
            story(
                21,
                "Ethics and Fairness",
                "Act IV: Real Systems",
                "Bias Review Chamber",
                "This lesson is about responsibility, not decorative policy language. It teaches how models inherit and amplify biases from data, labels, objectives, and deployment choices. I want you to leave with one clear instinct: average performance can hide serious harm.",
                vec![
                    dialogue("LYRA", "A model can be mathematically impressive and still be socially harmful."),
                    dialogue("LYRA", "That is why subgroup analysis, appeal paths, and documentation matter."),
                ],
            ),
            story(
                22,
                "Governance",
                "Act V: Control",
                "Governance Chamber",
                "Now we ask the question that ties the whole game together: who is accountable when a powerful AI system is deployed? This level teaches documentation, ownership, incident response, and operational control. A system is not mature just because it performs well. It is mature when people know who can inspect it, stop it, and justify it.",
                vec![
                    dialogue("LYRA", "Governance is how organizations prove they control the system rather than merely use it."),
                    dialogue("LYRA", "If no one owns the risk, the risk owns the organization."),
                ],
            ),
            story(
                23,
                "Scaling",
                "Act V: Control",
                "Distributed Systems Core",
                "We close the technical arc by looking at scaling. Frontier AI is a systems problem as much as an ML problem, which is why data throughput, checkpointing, communication, fault tolerance, and cost all become part of the real design conversation.",
                vec![
                    dialogue("LYRA", "Large models are not just larger ideas. They are larger coordination problems."),
                    dialogue("LYRA", "Capability gains only matter if the training and serving systems can sustain them."),
                ],
            ),
            story(
                24,
                "Recovery",
                "Act V: Control",
                "PROMETHEUS Core Interface",
                "At this point, you have traced the system from intelligence and learning through language, retrieval, agency, operations, security, and governance. PROMETHEUS should no longer feel like a mysterious entity. It is a stack you can reason about. That is what unlocks recovery: not faith in the machine, but understanding of how to build and constrain it.",
                vec![
                    dialogue("LYRA", "You were never here to admire PROMETHEUS. You were here to understand it well enough to control it responsibly."),
                    dialogue("LYRA", "That is the standard for real AI work: evidence, clarity, and disciplined trust."),
                ],
            ),
        ]
    }
}

impl Default for StoryEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn dialogue(speaker: &str, text: &str) -> Dialogue {
    Dialogue {
        speaker: speaker.to_string(),
        text: text.to_string(),
    }
}

fn story(
    level_id: usize,
    title: &str,
    act: &str,
    setting: &str,
    narrative: &str,
    character_dialogue: Vec<Dialogue>,
) -> StorySegment {
    StorySegment {
        level_id,
        title: title.to_string(),
        act: act.to_string(),
        setting: setting.to_string(),
        narrative: narrative.to_string(),
        character_dialogue,
    }
}

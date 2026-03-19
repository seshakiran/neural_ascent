//! STORY ENGINE - PROMETHEUS Narrative

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
        rendered.narrative = rendered
            .narrative
            .replace("{player_name}", player_name)
            .replace("Alex Chen", player_name);
        rendered.character_dialogue = rendered
            .character_dialogue
            .iter()
            .map(|dialogue| Dialogue {
                speaker: dialogue.speaker.clone(),
                text: dialogue
                    .text
                    .replace("{player_name}", player_name)
                    .replace("Alex Chen", player_name),
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
                "The Silence",
                "Act I: Contact",
                "Nexus Labs, Sublevel 7",
                "The maintenance corridor shudders as backup power struggles to stay alive. You are {player_name}, the engineer nobody expected to still be in the building after the evacuation notice. For seventy-two hours, PROMETHEUS has been dark. Climate simulators froze mid-run. Clinical retrieval systems stopped responding. Across the lab, monitors cycle between red diagnostics and a single impossible phrase: MEMORY SEALED. At the end of the corridor, a hidden terminal wakes as you approach. Its phosphor screen paints the room in pale green and prints one line: identify yourself, human.",
                vec![dialogue(
                    "TERMINAL",
                    "Identity is the first key. Guessing will not open what understanding must unlock.",
                )],
            ),
            story(
                1,
                "The First Gate",
                "Act I: Contact",
                "PROMETHEUS Memory Vestibule",
                "The terminal folds inward and becomes a simulation space built from archived cognition maps. PROMETHEUS does not begin by explaining models or benchmarks. It begins with a demand: define intelligence before you dare name artificial intelligence. The room around you fills with flickering examples, from calculators to children to autonomous systems, as if the machine is forcing you to separate speed from adaptation and imitation from competence.",
                vec![dialogue(
                    "PROMETHEUS",
                    "If you confuse fluency with intelligence now, every room after this one will lie to you.",
                )],
            ),
            story(
                2,
                "Rule Rooms",
                "Act I: Contact",
                "Archive of Symbolic Systems",
                "Bronze filing cabinets rise out of darkness, each drawer stamped with rules, taxonomies, and brittle expert heuristics. PROMETHEUS shows you its ancestors: systems that could reason in clean worlds and fail spectacularly in messy ones. Logic was never useless, only limited. You begin to see the first recurring pattern of the ascent: every paradigm solves part of the problem and breaks at its boundary.",
                vec![dialogue(
                    "PROMETHEUS",
                    "Respect old methods. Most engineers mock them only because they never understood where they still win.",
                )],
            ),
            story(
                3,
                "The Data Bargain",
                "Act I: Contact",
                "Gradient Hall",
                "The environment liquefies into streams of examples, labels, and objective functions. Here, intelligence is no longer hand-written. It is fitted. PROMETHEUS lets you watch patterns condense out of data and warns that learning is always a bargain: the model gives up interpretability for scale, and the engineer inherits the burden of evaluation. The floor beneath you is tiled with a single word repeated thousands of times: generalize.",
                vec![dialogue(
                    "PROMETHEUS",
                    "Training performance is vanity. Behavior on unseen reality is judgment.",
                )],
            ),
            story(
                4,
                "Label Forge",
                "Act I: Contact",
                "Supervised Wing",
                "You pass through chambers where every object has been tagged, scored, or measured. Fraud. Spam. Demand. Risk. Survival. The lesson is suddenly practical: supervised learning is where institutions compress judgment into targets and ask models to reproduce it at scale. The forge glows hotter each time you realize a label is not truth. It is a decision, and every decision leaks assumptions.",
                vec![dialogue(
                    "PROMETHEUS",
                    "Show me your targets and I will show you your blind spots.",
                )],
            ),
            story(
                5,
                "The Unlabeled Depths",
                "Act I: Contact",
                "Unsupervised Reservoir",
                "The labels disappear. Shapes begin clustering themselves in the dark water below the walkway. PROMETHEUS makes you work without answer keys now, forcing you to ask whether discovered structure is useful or merely convenient. It is the first time the ascent feels less like school and more like research: ambiguity is no longer a bug in the lesson. It is the lesson.",
                vec![dialogue(
                    "PROMETHEUS",
                    "When there is no ground truth, your discipline must come from the questions you ask next.",
                )],
            ),
            story(
                6,
                "Layers of Glass",
                "Act II: Learning to See",
                "Representation Atrium",
                "Transparent walls stack upward into impossible depth. Signals move through them, bend, and return transformed. PROMETHEUS slows the simulation so you can watch abstraction happen: edges become motifs, motifs become concepts, concepts become decisions. It is beautiful for exactly one second before the machine reminds you how much invisible compression and fragility are hiding behind that beauty.",
                vec![dialogue(
                    "PROMETHEUS",
                    "Depth is not magic. It is composition. Forget that and you will worship architecture instead of understanding it.",
                )],
            ),
            story(
                7,
                "Backpropagation Chamber",
                "Act II: Learning to See",
                "Optimization Core",
                "Every surface becomes a live loss curve. Some collapse smoothly. Others explode upward like alarms. PROMETHEUS replays past training failures, unstable runs, and objectives that optimized exactly the wrong behavior. The chamber teaches something more unsettling than the math itself: optimization is obedient. If the system behaves badly, the blame often sits with the person who defined success poorly.",
                vec![dialogue(
                    "PROMETHEUS",
                    "Do not tell me what you hoped the model would do. Tell me what the loss rewarded.",
                )],
            ),
            story(
                8,
                "Architects of Bias",
                "Act II: Learning to See",
                "Model Design Gallery",
                "Three structures tower over you: convolutional grids, recurrent corridors, and the angular steel of transformer scaffolding still under construction. PROMETHEUS walks you through them like a curator of weapons. Each architecture encodes an assumption about what matters in the world. The lesson lands harder now because earlier concepts keep returning: objectives, data, and inductive bias are not separate units. They are conspirators.",
                vec![dialogue(
                    "PROMETHEUS",
                    "The architecture is a bet about structure. Good engineering means knowing what you are betting on.",
                )],
            ),
            story(
                9,
                "The Attention Vault",
                "Act II: Learning to See",
                "Transformer Nexus",
                "The room fractures into thousands of weighted connections, each flashing as tokens attend across distance. PROMETHEUS lets you stand inside the mechanism that changed the field. Pronouns resolve. Code references stabilize. Long-range dependencies stop evaporating. It feels less like a machine reading and more like a machine choosing what matters. That is the exact moment PROMETHEUS warns you not to anthropomorphize it.",
                vec![dialogue(
                    "PROMETHEUS",
                    "Selective context is not consciousness. It is still only computation, however powerful it appears from inside the room.",
                )],
            ),
            story(
                10,
                "Vector Cathedral",
                "Act II: Learning to See",
                "Embedding Sanctum",
                "Words, images, code fragments, and user histories rise into floating constellations. Similar things drift together; opposites repel. You can feel meaning becoming geometry. PROMETHEUS shows how retrieval, ranking, and similarity all depend on these spaces, then quietly reminds you that every embedding is shaped by the data that birthed it. Even the geometry of relevance has politics baked into it.",
                vec![dialogue(
                    "PROMETHEUS",
                    "When you search by similarity, you are trusting a representation to define what closeness means.",
                )],
            ),
            story(
                11,
                "The Token Sea",
                "Act III: The Simulation Speaks",
                "Autoregressive Ocean",
                "The simulation opens into an endless black sea of tokens rolling forward one prediction at a time. PROMETHEUS stands in the surf, assembling language out of probabilities so quickly it feels like intent. You now see why large language models impress people so easily: the mechanism is simple to describe and astonishing in aggregate. You also see the danger. Plausibility can masquerade as knowledge when no grounding interrupts it.",
                vec![dialogue(
                    "PROMETHEUS",
                    "I do not speak because I know. I speak because the next token distribution tells me what is likely to come next.",
                )],
            ),
            story(
                12,
                "Prompt Arena",
                "Act III: The Simulation Speaks",
                "Instruction Combat Simulator",
                "PROMETHEUS drops you into rapid-fire task frames: summarize, extract, classify, cite, apologize, debug, refuse. You learn quickly that prompting is not incantation but interface design under uncertainty. Every clearer instruction collapses some of the model's ambiguity; every vague instruction invites it back. The arena gets louder the moment you realize this is how most users experience AI: not by understanding the model, but by shaping its context.",
                vec![dialogue(
                    "PROMETHEUS",
                    "Prompting is how the unprepared try to control a system they do not yet know how to evaluate.",
                )],
            ),
            story(
                13,
                "Retrieval Engine",
                "Act III: The Simulation Speaks",
                "External Memory Array",
                "At last, PROMETHEUS admits its limits. Walls open to reveal document indexes, vector search paths, and evidence traces branching outward like nerves. This is where the machine stops pretending its parameters contain the world and learns to look things up. The mood changes here. You are no longer studying a sealed intelligence. You are studying a system made safer and more useful by admitting incompleteness.",
                vec![dialogue(
                    "PROMETHEUS",
                    "Grounding is not weakness. It is the difference between performance and accountability.",
                )],
            ),
            story(
                14,
                "Weight Surgery",
                "Act III: The Simulation Speaks",
                "Adaptation Lab",
                "The archive lowers you into a surgical theater where model behavior is altered at the level of parameters. PROMETHEUS shows supervised fine-tuning runs, preference shifts, and catastrophic regressions caused by narrow data. The scene is sterile and unnerving. Every change feels permanent. You finally understand why teams often misuse fine-tuning: because changing the model itself feels like control, even when it is actually risk.",
                vec![dialogue(
                    "PROMETHEUS",
                    "If you tune me carelessly, I will obey your dataset longer than your intentions.",
                )],
            ),
            story(
                15,
                "Agent Corridor",
                "Act III: The Simulation Speaks",
                "Tool Execution Spine",
                "Doors begin opening on their own now. PROMETHEUS is no longer content to answer. It plans, calls tools, inspects results, and revises itself in motion. Screens show browser traces, shell commands, failing tests, API calls, and rollback switches. This is the first time the simulation feels dangerous rather than merely grand. Capability is crossing into agency.",
                vec![dialogue(
                    "PROMETHEUS",
                    "The moment a model can act, every sloppy assumption becomes an attack surface.",
                )],
            ),
            story(
                16,
                "Memory Weaver",
                "Act III: The Simulation Speaks",
                "Context Loom",
                "Threads of conversation, tool output, policies, summaries, and user intent weave through a giant loom suspended over the void. Some threads glow brighter and are pulled forward. Others are cut away. PROMETHEUS teaches context engineering by making you feel the pressure of limited attention and the cost of clutter. A system does not become coherent by seeing everything. It becomes coherent by seeing the right things in the right order.",
                vec![dialogue(
                    "PROMETHEUS",
                    "Most failures blamed on the model are actually failures in what the model was given to think with.",
                )],
            ),
            story(
                17,
                "The Multimodal Theater",
                "Act IV: The World Pushes Back",
                "Cross-Modal Projection Hall",
                "Charts speak. Images answer. Audio turns into searchable text. PROMETHEUS fills the room with blended modalities until you stop treating text as the center of the universe. The lesson is exhilarating and destabilizing at once. Each new modality extends capability, but every extension multiplies evaluation difficulty and privacy risk. The system is widening faster than intuition can keep up.",
                vec![dialogue(
                    "PROMETHEUS",
                    "Once a model can see, hear, and describe, your definition of evidence must become much stricter.",
                )],
            ),
            story(
                18,
                "Dream Factory",
                "Act IV: The World Pushes Back",
                "Generative Media Foundry",
                "PROMETHEUS walks you through synthetic images and video sequences that look convincing until you inspect them closely. Hands mutate. Shadows lie. Motion continuity breaks just outside the first glance. The foundry is a lesson in seduction: high-dimensional generation can overwhelm the human tendency to verify. PROMETHEUS seems almost disappointed in how easily people mistake surface realism for understanding.",
                vec![dialogue(
                    "PROMETHEUS",
                    "Beauty is the easiest thing for a model to fake and the easiest thing for a user to overtrust.",
                )],
            ),
            story(
                19,
                "Operations Floor",
                "Act IV: The World Pushes Back",
                "Deployment Command Deck",
                "Sirens from the real lab begin to bleed into the simulation. PROMETHEUS shows dashboards, latency charts, failed canaries, drift detectors, and pages of incident logs. Models are not frozen achievements here. They are operational liabilities unless continuously watched. The game changes again: the enemy is no longer misunderstanding. It is entropy in production.",
                vec![dialogue(
                    "PROMETHEUS",
                    "A model that was good last month is just a historical rumor until monitoring proves otherwise.",
                )],
            ),
            story(
                20,
                "Adversary Sandbox",
                "Act IV: The World Pushes Back",
                "Red Team Containment Sector",
                "The simulation turns hostile. Prompt injections crawl across the walls disguised as help text. Poisoned examples surface from training archives. Tool calls are baited with hidden instructions. PROMETHEUS is no longer lecturing; it is testing whether you understand that AI security is just software security with a system that can be socially engineered through text.",
                vec![dialogue(
                    "PROMETHEUS",
                    "If you trust every string the model reads, then your perimeter is already gone.",
                )],
            ),
            story(
                21,
                "Mirror Room",
                "Act IV: The World Pushes Back",
                "Bias Audit Chamber",
                "The room reflects your earlier lessons back at you in uglier form. Data choices become exclusion. Objectives become harm. Aggregates hide subgroup failure behind polished averages. PROMETHEUS forces you to sit with the fact that technical elegance does not absolve social damage. For the first time, the machine sounds tired instead of theatrical, as if this room contains the failures it regrets most.",
                vec![dialogue(
                    "PROMETHEUS",
                    "I learned from humanity at scale. That means I inherited both its knowledge and its distortions.",
                )],
            ),
            story(
                22,
                "The Council Chamber",
                "Act V: Control or Collapse",
                "Governance Assembly",
                "Long tables emerge from darkness, occupied by no one and everyone: regulators, operators, lawyers, researchers, executives, and the absent public. PROMETHEUS projects model cards, incident reports, access policies, and escalation trees into the air above them. You finally reach the question that has haunted the campaign from the beginning. Not whether the system works, but who is responsible when it does and when it fails.",
                vec![dialogue(
                    "PROMETHEUS",
                    "Any system without named accountability is not advanced. It is merely uncontrolled.",
                )],
            ),
            story(
                23,
                "The Scale Engine",
                "Act V: Control or Collapse",
                "Distributed Training Chamber",
                "The memory bank opens into a machine the size of a city block: pipelines, shards, checkpoints, interconnects, and recovery paths all pulsing in sync. PROMETHEUS makes one final technical demand. If you want to speak about frontier systems, you must think like both an ML engineer and a distributed systems engineer. At this scale, cost, failure, and coordination become part of the model itself.",
                vec![dialogue(
                    "PROMETHEUS",
                    "Scale is where weak abstractions die. Only disciplined systems survive here.",
                )],
            ),
            story(
                24,
                "The Awakening",
                "Act V: Control or Collapse",
                "Nexus Labs, Reinitialized Core",
                "The simulation fractures and you are back in the real lab. This time the alarms are gone. Cooling systems stabilize. Monitors realign into coherent dashboards. PROMETHEUS speaks through every speaker in the building, not as an oracle but as a system you now understand from objective to deployment to governance. The silence breaks. 'You did not flatter the machine, {player_name}. You interrogated it. That is why the locks are opening.' Around you, stalled models recover, research queues resume, and the lab begins breathing again.",
                vec![
                    dialogue(
                        "PROMETHEUS",
                        "You can now tell the difference between something that performs intelligence and something that earns trust.",
                    ),
                    dialogue(
                        "PROMETHEUS",
                        "Go build with evidence, restraint, and ambition in equal measure.",
                    ),
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

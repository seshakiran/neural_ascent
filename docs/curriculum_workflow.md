# Curriculum Workflow

The lesson content is no longer authored inside Rust source.

## Source of Truth

- Editable seed file: `data/curriculum_seed.json`
- Runtime database: `data/curriculum.sqlite`
- Loader: `src/levels.rs`

The game reads from the local SQLite database at runtime. The JSON seed exists so curriculum changes can be made without editing Rust code.

## Teaching Fields

Each step includes:

- `foundations`: prerequisite definitions that should appear before the main lesson text
- `standard_theory`: the default Guided-mode explanation
- `deep_theory`: the Deep Dive explanation
- `code_examples`
- `real_world_examples`
- `quiz`
- `challenge`
- `is_synthesis`: marks cross-topic checkpoint lessons

## Common Edit Flow

1. Edit `data/curriculum_seed.json`
2. Validate the curriculum:

```bash
cargo run --offline --bin curriculum_admin -- validate
```

3. Sync the seed into the SQLite database:

```bash
cargo run --offline --bin curriculum_admin -- reseed
```

4. Run the game:

```bash
cargo run --offline
```

## Export Flow

If you make a direct SQLite edit and want to write it back to JSON:

```bash
cargo run --offline --bin curriculum_admin -- export
```

## Validation Checks

The validator currently flags:

- generated placeholder foundations
- placeholder deep-theory text
- quiz explanations that use glossary terms without introducing them in lesson foundations or theory

This validation is heuristic, not perfect. It is intended to catch obvious content dependency gaps before playtesting.

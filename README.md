# Rust Steps

An adaptive, terminal-based Rust learning coach. It presents a lesson overview, goals, and a small example before each quiz. After you answer, it gives a score, explains the idea, identifies a concept to revisit, and recommends what to study next.

## Run

```sh
cargo run
```

Set `TYPESAFE_API_KEY` in your shell to enable TypeSafe Jev's structured score and concept diagnosis. Jev returns decisions, not prose; Rust Steps turns the result into written teaching feedback using its lesson guide. Without a key, the app labels and uses a basic local length-based score. Set `JEV_MODEL` to pin a model version if desired.

After every completed exercise, the app appends a progress record to the sibling journal repo, creates a readable Markdown page under `sessions/`, commits both artifacts, and pushes them to `origin`. Make sure the journal repo is cloned/available at `../rust-learning-journal` and has a working GitHub remote and authentication. Override the JSONL location with `RUST_COACH_JOURNAL=/path/to/journal/responses/session.jsonl`; with custom paths, the Markdown session folder and Git repo root are inferred from the same journal directory layout.

The exercise set is curated in the source code. `cargo run` presents one exercise per run; there is no automatic schedule.

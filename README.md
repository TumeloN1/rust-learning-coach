# Rust Steps

An adaptive, terminal-based Rust learning coach. It starts with core concepts and uses recent journal results to revisit topics where you scored below 3/5. Type answers in your own words; later exercises ask you to explain and write Rust code.

## Run

```sh
cargo run
```

Set `TYPESAFE_API_KEY` to enable the TypeSafe Jev grader. Jev is used for a structured understanding score and a primary concept diagnosis; the app turns those decisions into learner-friendly feedback. Without a key, a clearly labeled local fallback lets you explore the flow. Set `JEV_MODEL` to pin a model version if desired.

The default journal path points to the sibling learner repository. Override it with `RUST_COACH_JOURNAL=/path/to/session.jsonl`. Answers are saved as JSON Lines; avoid committing secrets or answers you want to keep private.

## Two-repository setup

This is the tool repository. The sibling `rust-learning-journal` repository stores your attempts and grader feedback. Configure a private remote for that repo if you want your learning history private, or make it public if you want a public exercise log. The app never commits or pushes your responses automatically.

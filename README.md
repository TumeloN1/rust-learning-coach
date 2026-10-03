# Rust Steps

An adaptive command-line Rust tutor. Every run begins with an in-depth topic overview, a concept walkthrough, learning goals, a code example, and a common pitfall before asking one quiz question. There are 26 curated exercises from beginner fundamentals through ownership, error handling, traits, testing, smart pointers, and concurrency.

## Run

```sh
cargo run
```

For TypeSafe grading, export `TYPESAFE_API_KEY` in the same Terminal session that runs `cargo run`. Jev provides the structured score and concept diagnosis. If the key is missing, invalid, or the request fails, the app says why and uses a clearly labeled local topic-keyword checklist rather than answer length.

For detailed answer-specific prose feedback, optionally set `OPENAI_API_KEY`. The app sends the exercise, answer, lesson notes, and structured assessment to OpenAI's Responses API. Set `OPENAI_MODEL` to choose a model (defaults to `gpt-6-astra`). If the key is absent, Rust Steps shows its built-in topic explanation and states that personalized prose feedback is off. Never commit API keys.

Each completed run creates a Markdown page under the sibling `rust-learning-journal/sessions/` folder. The page contains the full session and a hidden metadata comment used to adapt future exercise selection. The app commits and pushes that Markdown file to `origin`; it does not write JSONL. Keep the journal repository private because it contains your answers.

Set `RUST_COACH_JOURNAL=/path/to/rust-learning-journal` to change the journal repository location. For compatibility, an old path ending in `/responses/session.jsonl` or `/responses` is also accepted, but the app will still save Markdown only.

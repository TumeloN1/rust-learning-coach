use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

struct Exercise {
    id: &'static str,
    topic: &'static str,
    level: u8,
    prompt: &'static str,
    concepts: &'static [&'static str],
    hint: &'static str,
}
struct Lesson {
    overview: &'static str,
    goals: &'static [&'static str],
    example: &'static str,
    takeaway: &'static str,
}
const EXERCISES: &[Exercise] = &[
    Exercise {
        id: "01",
        topic: "variables",
        level: 1,
        prompt: "In your own words, explain what `let` does in Rust. How would you make a variable's value change?",
        concepts: &["variables", "mutability"],
        hint: "Rust bindings are immutable by default. What keyword allows reassignment?",
    },
    Exercise {
        id: "02",
        topic: "types",
        level: 1,
        prompt: "What is the difference between `i32` and `u32`? Give one situation where each is useful.",
        concepts: &["types", "integers"],
        hint: "Think about whether negative numbers are allowed.",
    },
    Exercise {
        id: "03",
        topic: "functions",
        level: 1,
        prompt: "Write a Rust function named `double` that takes an i32 and returns twice its value. Explain how Rust knows the return value.",
        concepts: &["functions", "expressions", "types"],
        hint: "A function signature includes parameter and return types. The final expression has no semicolon.",
    },
    Exercise {
        id: "04",
        topic: "ownership",
        level: 2,
        prompt: "What happens to a String after `let second = first;`? Explain why Rust behaves this way.",
        concepts: &["ownership", "move"],
        hint: "For a heap allocated String, assignment transfers ownership instead of copying its contents.",
    },
    Exercise {
        id: "05",
        topic: "borrowing",
        level: 2,
        prompt: "Explain the difference between `&String` and `&mut String`. What rule applies when mutable references are used?",
        concepts: &["borrowing", "references", "mutability"],
        hint: "At a time, you can have either one mutable reference or any number of immutable references.",
    },
    Exercise {
        id: "06",
        topic: "slices",
        level: 2,
        prompt: "What does `&s[0..2]` represent when `s` is a String, and why can a string slice be safer than copying text?",
        concepts: &["slices", "borrowing"],
        hint: "A slice borrows part of the original data and does not own it.",
    },
    Exercise {
        id: "07",
        topic: "enums",
        level: 3,
        prompt: "Describe how `Option<T>` represents a value that may be absent. Name its two variants and show a tiny example.",
        concepts: &["enums", "option", "pattern matching"],
        hint: "The variants are Some(value) and None; matching forces you to handle both cases.",
    },
    Exercise {
        id: "08",
        topic: "errors",
        level: 3,
        prompt: "When would you use `Result<T, E>` instead of `Option<T>`? What do the `?` operator and `unwrap()` each do?",
        concepts: &["result", "errors"],
        hint: "Result carries an error value. `?` returns errors to the caller; unwrap can panic.",
    },
    Exercise {
        id: "09",
        topic: "collections",
        level: 3,
        prompt: "How do you add an item to a Vec<i32>? What happens if you index a vector with an out-of-range position?",
        concepts: &["collections", "vectors", "panics"],
        hint: "Use push to add. Indexing panics; get returns an Option instead.",
    },
    Exercise {
        id: "10",
        topic: "traits",
        level: 4,
        prompt: "What problem do traits solve? Explain what it means for a type to implement a trait.",
        concepts: &["traits", "generics"],
        hint: "Traits describe shared behavior through required methods; types provide those methods.",
    },
    Exercise {
        id: "11",
        topic: "lifetimes",
        level: 4,
        prompt: "At a high level, what does a lifetime annotation like `'a` tell the compiler? Does it make a reference live longer?",
        concepts: &["lifetimes", "references"],
        hint: "Lifetimes describe relationships between reference scopes; they do not extend a value's lifetime.",
    },
    Exercise {
        id: "12",
        topic: "iterators",
        level: 4,
        prompt: "How is `.iter()` different from `.into_iter()` on a collection? What does `.map()` do?",
        concepts: &["iterators", "ownership", "closures"],
        hint: "Consider whether the iterator borrows items or takes ownership, then how map transforms each item.",
    },
];
#[derive(Serialize, Deserialize)]
struct Record {
    timestamp: u64,
    exercise_id: String,
    topic: String,
    level: u8,
    answer: String,
    score: u8,
    confidence: Option<f64>,
    diagnosis: String,
    feedback: String,
    next_exercise: String,
    grader: String,
}
fn lesson(topic: &str) -> Lesson {
    match topic {
        "variables" => Lesson {
            overview: "Rust gives each value a name through a binding. A binding created with `let` is immutable by default, which prevents accidental changes. Add `mut` only when you intend to reassign it.",
            goals: &[
                "Explain what a `let` binding names.",
                "Distinguish immutable and mutable bindings.",
                "Recognize that mutability is explicit.",
            ],
            example: "let score = 10;\nlet mut attempts = 0;\nattempts += 1;",
            takeaway: "Immutability is the default; `mut` is a deliberate opt-in.",
        },
        "types" => Lesson {
            overview: "Every Rust value has a type. Integer types with an `i` prefix are signed and can represent negative values; `u` types are unsigned and represent zero or positive values. The number gives the bit width. Rust can infer many types, but annotations make intent clear.",
            goals: &[
                "Read signed versus unsigned integer names.",
                "Choose a type based on the values it must represent.",
                "Understand why types help catch mistakes early.",
            ],
            example: "let temperature: i32 = -4;\nlet item_count: u32 = 12;",
            takeaway: "Pick a type whose allowed values fit the meaning of the data.",
        },
        "functions" => Lesson {
            overview: "Functions package reusable behavior. Rust function signatures state the types of inputs and the return type. A trailing expression without a semicolon becomes the return value; adding a semicolon turns it into a statement that returns unit `()`.",
            goals: &[
                "Identify function parameters and their types.",
                "Read an explicit return type.",
                "Tell a final expression from a statement ending in `;`.",
            ],
            example: "fn double(value: i32) -> i32 {\n    value * 2\n}",
            takeaway: "The last expression in a function body can be its return value.",
        },
        "ownership" => Lesson {
            overview: "Ownership is Rust's system for managing memory without a garbage collector. Each value has one owner. When a heap-owning value such as `String` is assigned to another binding, ownership moves; the old binding can no longer be used. Values with simple copy semantics, such as many integers, are copied instead.",
            goals: &[
                "State that each value has one owner at a time.",
                "Explain why assigning a String moves ownership.",
                "Recognize that a moved-from binding cannot be used.",
            ],
            example: "let first = String::from(\"hi\");\nlet second = first; // ownership moves\n// println!(\"{first}\"); // compile error",
            takeaway: "A move transfers responsibility for a value instead of duplicating its owned data.",
        },
        "borrowing" => Lesson {
            overview: "A reference lets code use a value without taking ownership. `&T` is an immutable borrow; `&mut T` is a mutable borrow. Rust prevents conflicting access: while a mutable reference is active, there cannot also be other references to that same value.",
            goals: &[
                "Explain borrowing without ownership transfer.",
                "Distinguish `&T` from `&mut T`.",
                "State the rule that prevents simultaneous conflicting borrows.",
            ],
            example: "let mut name = String::from(\"Ada\");\nlet view = &name;\nprintln!(\"{view}\");\nlet edit = &mut name;\nedit.push('!');",
            takeaway: "Many readers or one writer at a time keeps references safe.",
        },
        "slices" => Lesson {
            overview: "A slice is a borrowed view into a contiguous part of a collection. A string slice such as `&text[0..2]` points into the original string, so it does not own or copy those bytes. Its lifetime is tied to the source value. Rust string indices are byte offsets and must land on UTF-8 character boundaries.",
            goals: &[
                "Recognize a range slice such as `&text[start..end]`.",
                "Explain that slices borrow rather than own data.",
                "Remember that String ranges use UTF-8 byte boundaries.",
            ],
            example: "let text = String::from(\"hello\");\nlet first = &text[0..2]; // \"he\"",
            takeaway: "A slice is a view into existing data and cannot outlive its source.",
        },
        "enums" => Lesson {
            overview: "An enum defines a value that can be one of several variants. `Option<T>` is Rust's built-in way to represent something that may or may not exist: `Some(value)` carries a value, and `None` means absent. Pattern matching lets you handle each possibility explicitly.",
            goals: &[
                "Name the `Some(T)` and `None` variants.",
                "Explain why Option makes absence explicit.",
                "Use a match to handle both cases.",
            ],
            example: "let maybe_age: Option<u8> = Some(30);\nmatch maybe_age {\n    Some(age) => println!(\"{age}\"),\n    None => println!(\"unknown\"),\n}",
            takeaway: "Option makes the possibility of no value part of the type.",
        },
        "errors" => Lesson {
            overview: "Rust commonly represents recoverable failure with `Result<T, E>`: `Ok(value)` is success and `Err(error)` is failure. `Option<T>` represents presence or absence without an error detail. The `?` operator returns an error from the current function; `unwrap()` extracts a success value but panics on an error.",
            goals: &[
                "Distinguish absence (`Option`) from success-or-error (`Result`).",
                "Identify `Ok` and `Err`.",
                "Know that `?` propagates while `unwrap()` can panic.",
            ],
            example: "fn read_count() -> Result<u32, std::num::ParseIntError> {\n    let count = \"42\".parse::<u32>()?;\n    Ok(count)\n}",
            takeaway: "Prefer handling or propagating errors to panicking with `unwrap()` in normal application paths.",
        },
        "collections" => Lesson {
            overview: "A `Vec<T>` is a growable list of values with one element type. Use `push` to append. Indexing with `items[i]` panics if the index is out of bounds; `items.get(i)` returns `Option<&T>`, allowing safe handling when an element may be missing.",
            goals: &[
                "Recognize Vec as a growable, same-type collection.",
                "Append with `push`.",
                "Compare panicking indexing with the optional result from `get`.",
            ],
            example: "let mut values = vec![10, 20];\nvalues.push(30);\nif let Some(value) = values.get(4) {\n    println!(\"{value}\");\n}",
            takeaway: "Use `get` when an index may be missing and you want to handle that case.",
        },
        "traits" => Lesson {
            overview: "A trait describes behavior a type can provide, usually through method signatures. A type implements a trait by supplying those methods. Generic code can require a trait and then work with any type that provides the required behavior.",
            goals: &[
                "Describe a trait as shared behavior.",
                "Explain what implementing a trait means.",
                "See how a trait bound enables generic code.",
            ],
            example: "trait Speak { fn speak(&self); }\nstruct Cat;\nimpl Speak for Cat {\n    fn speak(&self) { println!(\"meow\"); }\n}",
            takeaway: "Traits let different types share an interface for behavior.",
        },
        "lifetimes" => Lesson {
            overview: "A lifetime describes how long a reference is valid and lets the compiler check that a reference never outlives the data it points to. An annotation such as `'a` names a relationship between reference scopes; it does not extend the life of any value. Many lifetimes are inferred.",
            goals: &[
                "Explain the purpose of lifetime checking.",
                "Read `'a` as a relationship between reference scopes.",
                "Avoid thinking annotations extend a value's lifetime.",
            ],
            example: "fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {\n    if left.len() >= right.len() { left } else { right }\n}",
            takeaway: "Lifetimes describe and check reference validity; they do not keep data alive.",
        },
        _ => Lesson {
            overview: "Iterators provide a way to process a sequence of values. `.iter()` borrows items, `.iter_mut()` mutably borrows them, and `.into_iter()` consumes the collection (the exact item type also depends on the receiver). Adapters such as `.map()` transform items lazily until a consuming operation runs.",
            goals: &[
                "Distinguish borrowed iteration from consuming a collection.",
                "Explain that `map` transforms each yielded item.",
                "Recognize that iterators are lazy until consumed.",
            ],
            example: "let values = vec![1, 2, 3];\nlet doubled: Vec<_> = values.iter().map(|n| n * 2).collect();",
            takeaway: "Choose the iterator method based on whether you want to borrow or consume the collection.",
        },
    }
}
fn input(prompt: &str) -> String {
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut s = String::new();
    if io::stdin().read_line(&mut s).is_err() {
        return String::new();
    }
    s.trim().to_owned()
}
fn record_path() -> PathBuf {
    PathBuf::from(
        env::var("RUST_COACH_JOURNAL")
            .unwrap_or_else(|_| "../rust-learning-journal/responses/session.jsonl".into()),
    )
}
fn history() -> Vec<Record> {
    let mut out = Vec::new();
    if let Ok(data) = fs::read_to_string(record_path()) {
        for line in data.lines() {
            if let Ok(r) = serde_json::from_str::<Record>(line) {
                out.push(r)
            }
        }
    }
    out
}
fn choose(topic: Option<&str>, level: u8) -> &'static Exercise {
    EXERCISES
        .iter()
        .find(|e| Some(e.topic) == topic)
        .or_else(|| EXERCISES.iter().find(|e| e.level <= level))
        .unwrap_or(&EXERCISES[0])
}
fn ask_jev(ex: &Exercise, answer: &str) -> Option<(u8, Option<f64>, String)> {
    let key = env::var("TYPESAFE_API_KEY").ok()?;
    let state = json!({"exercise":ex.prompt,"topic":ex.topic,"concepts":ex.concepts,"learner_answer":answer});
    let body = json!({"model":env::var("JEV_MODEL").unwrap_or_else(|_|"jev-latest".into()),"state":state,"questions":{
     "understanding":{"type":"score","instructions":"Score the learner's demonstrated Rust understanding using the ordered levels. Judge correctness, explanation, and misconceptions; be generous with a correct beginner answer.","criteria":["No relevant understanding or fundamentally incorrect","Some relevant idea, but major misconception or missing core point","Partly correct, but important detail is missing or confused","Mostly correct with minor omission","Correct, clear, and complete for a beginner"]},
     "main_gap":{"type":"choice","instructions":"Choose the primary concept this learner misunderstands or needs next. Choose none when no meaningful gap appears.","criteria":{"ownership":"ownership","borrowing":"borrowing","mutability":"mutability","types":"types","errors":"errors","syntax":"syntax","none":"none"}}
    }});
    let response = reqwest::blocking::Client::new()
        .post("https://api.typesafe.ai/v1/systemone")
        .bearer_auth(key)
        .json(&body)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .ok()?;
    if !response.status().is_success() {
        eprintln!(
            "Jev request failed ({}); using local fallback.",
            response.status()
        );
        return None;
    }
    let v: Value = response.json().ok()?;
    let answers = &v["answers"];
    let raw = answers["understanding"]["score"]
        .as_f64()
        .unwrap_or(1.0)
        .round() as u8;
    let confidence = answers["understanding"]["confidence"].as_f64();
    let diagnosis = answers["main_gap"]["choice"]
        .as_str()
        .unwrap_or("none")
        .to_owned();
    Some((raw.min(4) + 1, confidence, diagnosis))
}
fn detailed_feedback(
    score: u8,
    diagnosis: &str,
    ex: &Exercise,
    teaching: &Lesson,
    grader: &str,
) -> String {
    let gap = if diagnosis == "none" {
        format!(
            "Jev did not flag a single dominant gap. Keep practising the related ideas: {}.",
            ex.concepts.join(", ")
        )
    } else {
        format!(
            "The clearest area to revisit is **{diagnosis}**. {}",
            ex.hint
        )
    };
    let judgement = match score {
        5 => "Your answer shows a solid grasp of this lesson's main idea.",
        4 => "You have the core idea; add the missing detail called out below.",
        3 => {
            "You are partway there. Compare your answer with the walkthrough and focus on the distinction it highlights."
        }
        _ => {
            "This concept is still new. That's expected; use the walkthrough as a model and try a fresh explanation."
        }
    };
    format!(
        "## Assessment\n\n**Result: {score}/5.** {judgement}\n\n{gap}\n\n## A clear explanation\n\n{}\n\n## Example\n\n```rust\n{}\n```\n\n## Try this next\n\n{}\n\n## How the feedback was produced\n\n{grader} supplied the structured assessment. Rust Steps assembled this explanation and the learning recommendation from its lesson guide; Jev does not generate prose feedback.\n",
        teaching.takeaway,
        teaching.example,
        if score >= 4 {
            "Move on to the next lesson and write a small Rust example that uses this idea."
        } else {
            "Before the next run, explain the key idea in your own words without looking at the example, then compare your explanation with the guide."
        }
    )
}
fn display_timestamp(timestamp: u64) -> String {
    let seconds = timestamp as i64;
    let days = seconds.div_euclid(86_400);
    let day_seconds = seconds.rem_euclid(86_400);
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 }.div_euclid(146_097);
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += if month <= 2 { 1 } else { 0 };
    let hour = day_seconds / 3_600;
    let minute = day_seconds % 3_600 / 60;
    let second = day_seconds % 60;
    format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02} UTC")
}
fn markdown(r: &Record, ex: &Exercise, teaching: &Lesson, next: &Exercise) -> String {
    let confidence = r
        .confidence
        .map(|c| format!("{:.0}%", c * 100.0))
        .unwrap_or_else(|| "not available".into());
    format!(
        "# Session: {} — {}\n\n- **Date (UTC):** {}\n- **Exercise:** {}\n- **Level:** {}\n- **Score:** {}/5\n- **Jev confidence:** {}\n- **Primary gap:** {}\n- **Grader:** {}\n\n## What I was learning\n\n{}\n\n### Goals\n\n{}\n\n## Quiz prompt\n\n{}\n\n## My answer\n\n{}\n\n## Feedback\n\n{}\n\n## Next exercise\n\n**{}** — {}\n",
        r.topic,
        r.exercise_id,
        display_timestamp(r.timestamp),
        r.exercise_id,
        r.level,
        r.score,
        confidence,
        r.diagnosis,
        r.grader,
        teaching.overview,
        teaching
            .goals
            .iter()
            .map(|g| format!("- {g}"))
            .collect::<Vec<_>>()
            .join("\n"),
        ex.prompt,
        r.answer,
        r.feedback,
        next.topic,
        next.prompt
    )
}
fn commit_and_push(journal_root: &Path, jsonl: &Path, markdown_path: &Path) {
    let relative_jsonl = jsonl.strip_prefix(journal_root).unwrap_or(jsonl);
    let relative_md = markdown_path
        .strip_prefix(journal_root)
        .unwrap_or(markdown_path);
    let add = Command::new("git")
        .args(["-C"])
        .arg(journal_root)
        .args(["add"])
        .arg(relative_jsonl)
        .arg(relative_md)
        .status();
    if !matches!(add,Ok(s) if s.success()) {
        eprintln!("Journal saved locally, but git add failed; it was not pushed.");
        return;
    }
    let commit = Command::new("git")
        .args(["-C"])
        .arg(journal_root)
        .args(["commit", "-m", "Add Rust practice session"])
        .status();
    if !matches!(commit,Ok(s) if s.success()) {
        eprintln!("Journal saved locally, but commit failed; it was not pushed.");
        return;
    }
    let push = Command::new("git")
        .args(["-C"])
        .arg(journal_root)
        .args(["push", "origin", "HEAD"])
        .status();
    if matches!(push,Ok(s) if s.success()) {
        println!("Committed and pushed the Markdown session to the private journal repo.")
    } else {
        eprintln!(
            "Session is committed locally, but push failed. Check GitHub authentication and run `git -C {} push origin HEAD`.",
            journal_root.display()
        );
    }
}
fn main() {
    println!(
        "\n🦀 Rust Steps — your adaptive Rust practice coach\nType `quit` at the answer prompt to stop. Completed sessions are saved to your learner journal.\n"
    );
    let history = history();
    let weak = history.iter().find(|r| r.score < 3);
    let selected = if let Some(r) = weak {
        choose(Some(&r.topic), r.level)
    } else if let Some(last) = history.last() {
        EXERCISES
            .iter()
            .find(|e| e.id == last.next_exercise)
            .unwrap_or(&EXERCISES[0])
    } else {
        &EXERCISES[0]
    };
    let teaching = lesson(selected.topic);
    println!(
        "══════════════════════════════════════════════════════════\nBEFORE THE QUIZ · {} · level {}\n══════════════════════════════════════════════════════════\n\n{}\n\nBy the end, you should be able to:\n{}\n\nExample:\n```rust\n{}\n```\n\nKey takeaway: {}\n\nWhen you're ready, continue to the quiz. No rush; you can review this guide first.\n",
        selected.topic,
        selected.level,
        teaching.overview,
        teaching
            .goals
            .iter()
            .map(|g| format!("  • {g}"))
            .collect::<Vec<_>>()
            .join("\n"),
        teaching.example,
        teaching.takeaway
    );
    let _ = input("Press Enter when ready...");
    println!("\nQUIZ · Exercise {}\n{}\n", selected.id, selected.prompt);
    println!("Tip: answer in your own words. For code prompts, include a code snippet.");
    let answer = input("\nYour answer> ");
    if answer.eq_ignore_ascii_case("quit") {
        return;
    }
    if answer.is_empty() {
        println!("No answer recorded.");
        return;
    }
    let result = ask_jev(selected, &answer);
    let (score, confidence, diagnosis, grader) = if let Some((s, c, d)) = result {
        (
            s,
            c,
            d,
            "TypeSafe Jev + Rust Steps lesson guide".to_string(),
        )
    } else {
        let s = if answer.len() > 100 {
            3
        } else if answer.len() > 40 {
            2
        } else {
            1
        };
        (s,None,"needs review".into(),"local length-based fallback + Rust Steps lesson guide (set TYPESAFE_API_KEY to enable Jev)".into())
    };
    let fb = detailed_feedback(score, &diagnosis, selected, &teaching, &grader);
    let next = if score >= 4 {
        EXERCISES
            .iter()
            .find(|e| e.level > selected.level)
            .unwrap_or(&EXERCISES[EXERCISES.len() - 1])
    } else {
        choose(Some(selected.topic), selected.level)
    };
    println!(
        "\n{fb}\nSuggested next exercise: {} — {}\n",
        next.topic, next.prompt
    );
    let r = Record {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        exercise_id: selected.id.into(),
        topic: selected.topic.into(),
        level: selected.level,
        answer,
        score,
        confidence,
        diagnosis,
        feedback: fb,
        next_exercise: next.id.into(),
        grader,
    };
    let path = record_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let line = match serde_json::to_string(&r) {
        Ok(line) => line,
        Err(e) => {
            eprintln!("Could not encode journal record: {e}");
            return;
        }
    };
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&path) {
        if writeln!(f, "{line}").is_err() {
            eprintln!("Could not save journal record.");
            return;
        }
    } else {
        eprintln!("Could not save journal record to {}", path.display());
        return;
    }
    let journal_root = path
        .parent()
        .unwrap_or(Path::new("."))
        .parent()
        .unwrap_or(Path::new("."));
    let md_path = journal_root
        .join("sessions")
        .join(format!("session-{}-{}.md", r.timestamp, r.topic));
    if let Some(parent) = md_path.parent() {
        if fs::create_dir_all(parent).is_err() {
            eprintln!("Saved progress record, but could not create Markdown session directory.");
            return;
        }
    }
    if fs::write(&md_path, markdown(&r, selected, &teaching, next)).is_err() {
        eprintln!("Saved progress record, but could not write Markdown session.");
        return;
    }
    println!("Saved readable session notes to {}", md_path.display());
    commit_and_push(journal_root, &path, &md_path);
}

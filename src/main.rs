use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{env, fs::{self, OpenOptions}, io::{self, Write}, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

#[derive(Clone)]
struct Exercise { id: &'static str, topic: &'static str, level: u8, prompt: &'static str, concepts: &'static [&'static str], hint: &'static str }
const EXERCISES: &[Exercise] = &[
 Exercise{id:"01",topic:"variables",level:1,prompt:"In your own words, explain what `let` does in Rust. How would you make a variable's value change?",concepts:&["variables","mutability"],hint:"Rust bindings are immutable by default. What keyword allows reassignment?"},
 Exercise{id:"02",topic:"types",level:1,prompt:"What is the difference between `i32` and `u32`? Give one situation where each is useful.",concepts:&["types","integers"],hint:"Think about whether negative numbers are allowed."},
 Exercise{id:"03",topic:"functions",level:1,prompt:"Write a Rust function named `double` that takes an i32 and returns twice its value. Explain how Rust knows the return value.",concepts:&["functions","expressions","types"],hint:"A function signature includes parameter and return types. The final expression has no semicolon."},
 Exercise{id:"04",topic:"ownership",level:2,prompt:"What happens to a String after `let second = first;`? Explain why Rust behaves this way.",concepts:&["ownership","move"],hint:"For a heap allocated String, assignment transfers ownership instead of copying its contents."},
 Exercise{id:"05",topic:"borrowing",level:2,prompt:"Explain the difference between `&String` and `&mut String`. What rule applies when mutable references are used?",concepts:&["borrowing","references","mutability"],hint:"At a time, you can have either one mutable reference or any number of immutable references."},
 Exercise{id:"06",topic:"slices",level:2,prompt:"What does `&s[0..2]` represent when `s` is a String, and why can a string slice be safer than copying text?",concepts:&["slices","borrowing"],hint:"A slice borrows part of the original data and does not own it."},
 Exercise{id:"07",topic:"enums",level:3,prompt:"Describe how `Option<T>` represents a value that may be absent. Name its two variants and show a tiny example.",concepts:&["enums","option","pattern matching"],hint:"The variants are Some(value) and None; matching forces you to handle both cases."},
 Exercise{id:"08",topic:"errors",level:3,prompt:"When would you use `Result<T, E>` instead of `Option<T>`? What do the `?` operator and `unwrap()` each do?",concepts:&["result","errors"],hint:"Result carries an error value. `?` returns errors to the caller; unwrap can panic."},
 Exercise{id:"09",topic:"collections",level:3,prompt:"How do you add an item to a Vec<i32>? What happens if you index a vector with an out-of-range position?",concepts:&["collections","vectors","panics"],hint:"Use push to add. Indexing panics; get returns an Option instead."},
 Exercise{id:"10",topic:"traits",level:4,prompt:"What problem do traits solve? Explain what it means for a type to implement a trait.",concepts:&["traits","generics"],hint:"Traits describe shared behavior through required methods; types provide those methods."},
 Exercise{id:"11",topic:"lifetimes",level:4,prompt:"At a high level, what does a lifetime annotation like `'a` tell the compiler? Does it make a reference live longer?",concepts:&["lifetimes","references"],hint:"Lifetimes describe relationships between reference scopes; they do not extend a value's lifetime."},
 Exercise{id:"12",topic:"iterators",level:4,prompt:"How is `.iter()` different from `.into_iter()` on a collection? What does `.map()` do?",concepts:&["iterators","ownership","closures"],hint:"Consider whether the iterator borrows items or takes ownership, then how map transforms each item."},
];
#[derive(Serialize, Deserialize)] struct Record { timestamp: u64, exercise_id: String, topic: String, level: u8, answer: String, score: u8, confidence: Option<f64>, diagnosis: String, feedback: String, next_exercise: String, grader: String }
fn input(prompt: &str) -> String { print!("{prompt}"); let _=io::stdout().flush(); let mut s=String::new(); if io::stdin().read_line(&mut s).is_err(){return String::new()} s.trim().to_owned() }
fn record_path() -> PathBuf { PathBuf::from(env::var("RUST_COACH_JOURNAL").unwrap_or_else(|_| "../rust-learning-journal/responses/session.jsonl".into())) }
fn prior_topics() -> Vec<String> { let mut out=Vec::new(); if let Ok(data)=fs::read_to_string(record_path()){for line in data.lines(){if let Ok(r)=serde_json::from_str::<Record>(line){if r.score<3 {out.push(r.topic)}}}} out }
fn choose(topic: Option<&str>, level: u8) -> &'static Exercise { EXERCISES.iter().find(|e| Some(e.topic)==topic).or_else(|| EXERCISES.iter().find(|e| e.level<=level)).unwrap_or(&EXERCISES[0]) }
fn ask_jev(ex: &Exercise, answer: &str) -> Option<(u8, Option<f64>, String)> {
 let key=env::var("TYPESAFE_API_KEY").ok()?;
 let state=json!({"exercise":ex.prompt,"topic":ex.topic,"concepts":ex.concepts,"learner_answer":answer});
 let body=json!({"model":env::var("JEV_MODEL").unwrap_or_else(|_|"jev-latest".into()),"state":state,"questions":{
  "understanding":{"type":"score","instructions":"Score the learner's demonstrated Rust understanding using the ordered levels. Judge correctness, explanation, and misconceptions; be generous with a correct beginner answer.","criteria":["No relevant understanding or fundamentally incorrect","Some relevant idea, but major misconception or missing core point","Partly correct, but important detail is missing or confused","Mostly correct with minor omission","Correct, clear, and complete for a beginner"]},
  "main_gap":{"type":"choice","instructions":"Choose the primary concept this learner misunderstands or needs next. Choose none when no meaningful gap appears.","criteria":{"ownership":"ownership","borrowing":"borrowing","mutability":"mutability","types":"types","errors":"errors","syntax":"syntax","none":"none"}}
 }});
 let response=reqwest::blocking::Client::new().post("https://api.typesafe.ai/v1/systemone").bearer_auth(key).json(&body).timeout(std::time::Duration::from_secs(30)).send().ok()?;
 if !response.status().is_success(){eprintln!("Jev request failed ({}); using local feedback.",response.status());return None}
 let v:Value=response.json().ok()?; let answers=&v["answers"];
 let raw=answers["understanding"]["score"].as_f64().unwrap_or(1.0).round() as u8;
 let confidence=answers["understanding"]["confidence"].as_f64();
 let diagnosis=answers["main_gap"]["choice"].as_str().unwrap_or("none").to_owned();
 Some((raw.min(4)+1,confidence,diagnosis))
}
fn feedback(score:u8, topic:&str, diagnosis:&str, answer:&str, hint:&str)->String {
 if answer.trim().len()<12 {return format!("Try adding a sentence that explains your reasoning. Hint: {hint}")}
 match score { 5=>format!("Strong answer. You explained the key idea for {topic} clearly. Next, try applying it in a small Rust program."), 4=>format!("You have the main idea. Tighten one detail about {topic}, then try this: {hint}"), _=>format!("Let's strengthen {topic}. Jev flagged `{diagnosis}` as the main gap. {hint} Write a revised explanation in your own words.") }
}
fn main(){
 println!("\n🦀 Rust Steps — a small, adaptive Rust practice coach\nType `quit` to stop. Progress is saved to your learner journal.\n");
 let weak=prior_topics(); let level=if weak.is_empty(){1}else{2}; let selected=if let Some(t)=weak.first(){choose(Some(t),level)}else{choose(None,level)};
 println!("Exercise {} · {} · level {}\n{}\n",selected.id,selected.topic,selected.level,selected.prompt);
 println!("Tip: answer in your own words. For code prompts, include a code snippet.");
 let answer=input("\nYour answer> "); if answer.eq_ignore_ascii_case("quit"){return} if answer.is_empty(){println!("No answer recorded.");return}
 let result=ask_jev(selected,&answer); let (score,confidence,diagnosis,grader)=if let Some((s,c,d))=result{(s,c,d,"TypeSafe Jev + Rust Steps".to_string())}else{let s=if answer.len()>100{3}else if answer.len()>40{2}else{1};(s,None,"needs review".into(),"local fallback (set TYPESAFE_API_KEY to enable Jev)".into())};
 let fb=feedback(score,selected.topic,&diagnosis,&answer,selected.hint);
 let next=if score>=4 {EXERCISES.iter().find(|e|e.level>selected.level).unwrap_or(&EXERCISES[EXERCISES.len()-1])} else {choose(Some(selected.topic),selected.level)};
 println!("\nScore: {score}/5 · Grader: {grader}"); if let Some(c)=confidence{println!("Jev confidence: {:.0}%",c*100.0)} println!("Feedback: {fb}\nNext up: {} — {}\n",next.topic,next.prompt);
 let r=Record{timestamp:SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),exercise_id:selected.id.into(),topic:selected.topic.into(),level:selected.level,answer,score,confidence,diagnosis,feedback:fb,next_exercise:next.id.into(),grader};
 let path=record_path(); if let Some(parent)=path.parent(){let _=fs::create_dir_all(parent);} if let Ok(mut f)=OpenOptions::new().create(true).append(true).open(&path){if let Ok(line)=serde_json::to_string(&r){let _=writeln!(f,"{line}");println!("Saved to {}",path.display());}}else{eprintln!("Could not save journal entry to {}",path.display());}
}

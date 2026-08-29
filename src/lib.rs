use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureKind {
    Input,
    Safety,
    Execution,
}

#[derive(Debug)]
pub struct DrillFailure {
    kind: FailureKind,
    message: String,
}

impl DrillFailure {
    fn input(message: impl Into<String>) -> Self {
        Self {
            kind: FailureKind::Input,
            message: message.into(),
        }
    }

    fn safety(message: impl Into<String>) -> Self {
        Self {
            kind: FailureKind::Safety,
            message: message.into(),
        }
    }

    fn execution(message: impl Into<String>) -> Self {
        Self {
            kind: FailureKind::Execution,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for DrillFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(formatter)
    }
}

impl std::error::Error for DrillFailure {}

pub fn failure_exit_code(error: &anyhow::Error) -> i32 {
    match error
        .chain()
        .find_map(|cause| cause.downcast_ref::<DrillFailure>())
    {
        Some(DrillFailure {
            kind: FailureKind::Safety,
            ..
        }) => 3,
        Some(DrillFailure {
            kind: FailureKind::Execution,
            ..
        }) => 4,
        Some(DrillFailure {
            kind: FailureKind::Input,
            ..
        }) => 2,
        None => 2,
    }
}

#[derive(Debug, Deserialize)]
struct Workflow {
    jobs: BTreeMap<String, Job>,
}

#[derive(Debug, Deserialize)]
struct Job {
    #[serde(rename = "runs-on")]
    runs_on: Option<Value>,
    container: Option<Value>,
    needs: Option<Value>,
    #[serde(default)]
    env: BTreeMap<String, Value>,
    #[serde(default)]
    steps: Vec<Step>,
}

#[derive(Debug, Deserialize)]
struct Step {
    name: Option<String>,
    uses: Option<String>,
    run: Option<String>,
    #[serde(default)]
    env: BTreeMap<String, Value>,
    #[serde(rename = "working-directory")]
    working_directory: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DrillOptions {
    pub workflow: PathBuf,
    pub job: String,
    pub image: String,
    pub out: PathBuf,
    pub repo: PathBuf,
    pub allow_release: bool,
    pub execute: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrillReport {
    pub schema_version: u8,
    pub job: String,
    pub image: String,
    pub ready: bool,
    pub executed: bool,
    pub execution_passed: Option<bool>,
    pub commands_included: usize,
    pub commands_blocked: usize,
    pub secret_inputs: usize,
    pub required_files: Vec<String>,
    pub network_hosts: Vec<String>,
    pub provider_assumptions: Vec<String>,
    pub warnings: Vec<String>,
    pub packet_path: String,
}

impl DrillReport {
    pub fn summary(&self) -> String {
        let state = if self.execution_passed == Some(true) {
            "PASS"
        } else if self.ready {
            "READY"
        } else {
            "BLOCKED"
        };
        format!(
            "{state}  {}\npacket  {}\nsteps   {} included, {} blocked\ninputs  {} anonymous input(s)\n",
            self.job,
            self.packet_path,
            self.commands_included,
            self.commands_blocked,
            self.secret_inputs
        )
    }
}

fn yaml_string(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn is_pinned_image(image: &str) -> bool {
    let Some((_, digest)) = image.rsplit_once("@sha256:") else {
        return false;
    };
    digest.len() == 64 && digest.chars().all(|c| c.is_ascii_hexdigit())
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn shell_env_value(value: &str) -> String {
    let variable = Regex::new(r"\$(DRILL_(?:SECRET|CONTEXT)_[A-Za-z0-9_]+)").unwrap();
    let mut parts = Vec::new();
    let mut start = 0;
    for capture in variable.captures_iter(value) {
        let whole = capture.get(0).unwrap();
        if whole.start() > start {
            parts.push(shell_quote(&value[start..whole.start()]));
        }
        parts.push(format!("\"${}\"", &capture[1]));
        start = whole.end();
    }
    if start < value.len() {
        parts.push(shell_quote(&value[start..]));
    }
    if parts.is_empty() {
        shell_quote(value)
    } else {
        parts.join("")
    }
}

#[derive(Clone, Copy, Default)]
struct CommandFacts {
    release: bool,
    npm: bool,
    cargo: bool,
    pip: bool,
    docker: bool,
    network_unknown: bool,
}

impl CommandFacts {
    fn merge(&mut self, other: Self) {
        self.release |= other.release;
        self.npm |= other.npm;
        self.cargo |= other.cargo;
        self.pip |= other.pip;
        self.docker |= other.docker;
        self.network_unknown |= other.network_unknown;
    }
}

fn is_assignment(word: &str) -> bool {
    let Some((name, _)) = word.split_once('=') else {
        return false;
    };
    let mut characters = name.chars();
    matches!(characters.next(), Some(first) if first == '_' || first.is_ascii_alphabetic())
        && characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
}

/// Split shell source without evaluating it. Quoted text stays in one word so
/// `sh -c 'npm publish'` can be inspected recursively. An unfinished quote or
/// escape returns `None` and is treated as an unsafe, unparseable command.
fn shell_commands(source: &str) -> Option<Vec<Vec<String>>> {
    let normalized = source.replace("\\\n", " ");
    let mut commands = Vec::new();
    let mut words = Vec::new();
    let mut word = String::new();
    let mut quote = None;
    let mut escaped = false;
    let mut in_word = false;

    let finish_word = |words: &mut Vec<String>, word: &mut String, in_word: &mut bool| {
        if *in_word {
            words.push(std::mem::take(word));
            *in_word = false;
        }
    };
    let finish_command = |commands: &mut Vec<Vec<String>>, words: &mut Vec<String>| {
        if !words.is_empty() {
            commands.push(std::mem::take(words));
        }
    };

    for character in normalized.chars() {
        if escaped {
            word.push(character);
            in_word = true;
            escaped = false;
            continue;
        }
        match quote {
            Some('\'') => {
                if character == '\'' {
                    quote = None;
                } else {
                    word.push(character);
                }
                in_word = true;
            }
            Some('"') => {
                if character == '"' {
                    quote = None;
                } else if character == '\\' {
                    escaped = true;
                } else {
                    word.push(character);
                }
                in_word = true;
            }
            Some(_) => unreachable!(),
            None => match character {
                '\'' | '"' => {
                    quote = Some(character);
                    in_word = true;
                }
                '\\' => {
                    escaped = true;
                    in_word = true;
                }
                ' ' | '\t' | '\r' => finish_word(&mut words, &mut word, &mut in_word),
                '\n' | ';' | '|' | '&' => {
                    finish_word(&mut words, &mut word, &mut in_word);
                    finish_command(&mut commands, &mut words);
                }
                '(' | ')' => {
                    finish_word(&mut words, &mut word, &mut in_word);
                    finish_command(&mut commands, &mut words);
                }
                _ => {
                    word.push(character);
                    in_word = true;
                }
            },
        }
    }
    if escaped || quote.is_some() {
        return None;
    }
    finish_word(&mut words, &mut word, &mut in_word);
    finish_command(&mut commands, &mut words);
    Some(commands)
}

fn executable_name(word: &str) -> String {
    word.rsplit('/').next().unwrap_or(word).to_ascii_lowercase()
}

fn unknown_wrapper() -> CommandFacts {
    // A wrapper whose target cannot be determined is omitted from the packet.
    // This fail-closed result prevents dynamic or malformed wrappers from
    // bypassing the explicit release opt-in.
    CommandFacts {
        release: true,
        network_unknown: true,
        ..CommandFacts::default()
    }
}

fn wrapped_target<'a>(words: &'a [String], wrapper: &str) -> Result<&'a [String], ()> {
    let mut index = 1;
    while index < words.len() && is_assignment(&words[index]) {
        index += 1;
    }

    match wrapper {
        "env" => {
            while index < words.len() {
                let word = words[index].as_str();
                if word == "--" {
                    index += 1;
                    break;
                }
                if is_assignment(word) {
                    index += 1;
                    continue;
                }
                if matches!(word, "-u" | "--unset" | "-C" | "--chdir") {
                    index += 2;
                    continue;
                }
                if word.starts_with("--unset=") || word.starts_with("--chdir=") {
                    index += 1;
                    continue;
                }
                if matches!(word, "-i" | "--ignore-environment" | "-0" | "--null") {
                    index += 1;
                    continue;
                }
                // `env -S` reparses a string using implementation-specific
                // rules, so it cannot be trusted as a transparent wrapper.
                if matches!(word, "-S" | "--split-string")
                    || word.starts_with("--split-string=")
                    || word.starts_with('-')
                {
                    return Err(());
                }
                break;
            }
        }
        "command" => {
            while index < words.len() {
                match words[index].as_str() {
                    "--" | "-p" => index += 1,
                    "-v" | "-V" => return Ok(&[]),
                    word if word.starts_with('-') => return Err(()),
                    _ => break,
                }
            }
        }
        "exec" => {
            while index < words.len() {
                match words[index].as_str() {
                    "--" | "-c" | "-l" => index += 1,
                    "-a" => index += 2,
                    word if word.starts_with('-') => return Err(()),
                    _ => break,
                }
            }
        }
        "sudo" => {
            while index < words.len() {
                let word = words[index].as_str();
                if word == "--" {
                    index += 1;
                    break;
                }
                if is_assignment(word) {
                    index += 1;
                    continue;
                }
                if matches!(
                    word,
                    "-u" | "--user"
                        | "-g"
                        | "--group"
                        | "-h"
                        | "--host"
                        | "-p"
                        | "--prompt"
                        | "-C"
                        | "--close-from"
                        | "-r"
                        | "--role"
                        | "-t"
                        | "--type"
                        | "-T"
                        | "--command-timeout"
                        | "-D"
                        | "--chdir"
                ) {
                    index += 2;
                    continue;
                }
                if word.starts_with("--user=")
                    || word.starts_with("--group=")
                    || word.starts_with("--host=")
                    || word.starts_with("--prompt=")
                    || word.starts_with("--close-from=")
                    || word.starts_with("--role=")
                    || word.starts_with("--type=")
                    || word.starts_with("--command-timeout=")
                    || word.starts_with("--chdir=")
                {
                    index += 1;
                    continue;
                }
                if matches!(
                    word,
                    "-A" | "--askpass"
                        | "-b"
                        | "--background"
                        | "-E"
                        | "--preserve-env"
                        | "-H"
                        | "--set-home"
                        | "-n"
                        | "--non-interactive"
                        | "-P"
                        | "--preserve-groups"
                        | "-S"
                        | "--stdin"
                ) || word.starts_with("--preserve-env=")
                {
                    index += 1;
                    continue;
                }
                if word.starts_with('-') {
                    return Err(());
                }
                break;
            }
        }
        "nohup" => {
            if words.get(index).is_some_and(|word| word == "--") {
                index += 1;
            } else if words.get(index).is_some_and(|word| word.starts_with('-')) {
                return Err(());
            }
        }
        "nice" => {
            while index < words.len() {
                let word = words[index].as_str();
                if word == "--" {
                    index += 1;
                    break;
                }
                if matches!(word, "-n" | "--adjustment") {
                    index += 2;
                    continue;
                }
                if word.starts_with("--adjustment=")
                    || (word.starts_with('-')
                        && word.len() > 1
                        && word[1..]
                            .chars()
                            .all(|character| character.is_ascii_digit()))
                {
                    index += 1;
                    continue;
                }
                if word.starts_with('-') {
                    return Err(());
                }
                break;
            }
        }
        "time" => {
            while index < words.len() {
                let word = words[index].as_str();
                if word == "--" {
                    index += 1;
                    break;
                }
                if matches!(word, "-f" | "--format" | "-o" | "--output") {
                    index += 2;
                    continue;
                }
                if matches!(
                    word,
                    "-a" | "--append" | "-p" | "--portability" | "--quiet" | "-v" | "--verbose"
                ) || word.starts_with("--format=")
                    || word.starts_with("--output=")
                {
                    index += 1;
                    continue;
                }
                if word.starts_with('-') {
                    return Err(());
                }
                break;
            }
        }
        "timeout" => {
            while index < words.len() {
                let word = words[index].as_str();
                if word == "--" {
                    index += 1;
                    break;
                }
                if matches!(word, "-k" | "--kill-after" | "-s" | "--signal") {
                    index += 2;
                    continue;
                }
                if matches!(
                    word,
                    "--preserve-status" | "--foreground" | "-v" | "--verbose"
                ) || word.starts_with("--kill-after=")
                    || word.starts_with("--signal=")
                {
                    index += 1;
                    continue;
                }
                if word.starts_with('-') {
                    return Err(());
                }
                break;
            }
            // The first non-option is timeout's duration, not its target.
            index += 1;
        }
        _ => return Err(()),
    }

    if index < words.len() {
        Ok(&words[index..])
    } else {
        Err(())
    }
}

fn facts_for_words(words: &[String], depth: usize) -> CommandFacts {
    if words.is_empty() {
        return CommandFacts::default();
    }
    if depth > 8 {
        return unknown_wrapper();
    }

    let mut start = 0;
    while start < words.len() && is_assignment(&words[start]) {
        start += 1;
    }
    if start == words.len() {
        return CommandFacts::default();
    }
    let words = &words[start..];
    let executable = executable_name(&words[0]);
    if executable.contains('$') || executable.contains('`') {
        return unknown_wrapper();
    }

    if matches!(
        executable.as_str(),
        "env" | "command" | "exec" | "sudo" | "nohup" | "nice" | "time" | "timeout"
    ) {
        return match wrapped_target(words, &executable) {
            Ok([]) => CommandFacts::default(),
            Ok(target) => facts_for_words(target, depth + 1),
            Err(()) => unknown_wrapper(),
        };
    }

    if matches!(executable.as_str(), "sh" | "bash" | "dash" | "ksh" | "zsh") {
        let command_index = words.iter().enumerate().skip(1).find_map(|(index, word)| {
            let grouped_short_flags = word
                .strip_prefix('-')
                .filter(|flags| !flags.starts_with('-') && flags.len() > 1)
                .is_some_and(|flags| {
                    flags.contains('c')
                        && flags.chars().all(|flag| {
                            matches!(
                                flag,
                                'a' | 'b'
                                    | 'c'
                                    | 'e'
                                    | 'f'
                                    | 'h'
                                    | 'k'
                                    | 'l'
                                    | 'm'
                                    | 'n'
                                    | 'p'
                                    | 't'
                                    | 'u'
                                    | 'v'
                                    | 'x'
                                    | 'B'
                                    | 'C'
                                    | 'E'
                                    | 'H'
                                    | 'P'
                                    | 'T'
                            )
                        })
                });
            (word == "-c" || grouped_short_flags).then_some(index + 1)
        });
        if let Some(index) = command_index {
            return words
                .get(index)
                .map(|source| command_facts_at_depth(source, depth + 1))
                .unwrap_or_else(unknown_wrapper);
        }
    }
    if executable == "eval" {
        return if words.len() > 1 {
            command_facts_at_depth(&words[1..].join(" "), depth + 1)
        } else {
            unknown_wrapper()
        };
    }

    let rest = &words[1..];
    let has = |word: &str| {
        rest.iter()
            .any(|argument| argument.eq_ignore_ascii_case(word))
    };
    let npm_publish = (executable == "npm" && (has("publish") || has("pub")))
        || (executable == "pnpm" && has("publish"))
        || (executable == "yarn"
            && rest.len() >= 2
            && rest[0].eq_ignore_ascii_case("npm")
            && (rest[1].eq_ignore_ascii_case("publish") || rest[1].eq_ignore_ascii_case("pub")));
    let release = npm_publish
        || (executable == "cargo" && has("publish"))
        || (executable == "docker" && has("push"))
        || (executable == "gh"
            && rest.len() >= 2
            && rest[0].eq_ignore_ascii_case("release")
            && rest[1].eq_ignore_ascii_case("create"))
        || (executable == "git"
            && has("push")
            && rest
                .iter()
                .any(|argument| *argument == "--tags" || *argument == "--follow-tags"))
        || (executable == "twine" && has("upload"))
        || (executable == "gem" && has("push"))
        || (executable == "helm" && has("push"))
        || (executable == "kubectl" && has("apply"))
        || (executable == "terraform" && has("apply"));

    CommandFacts {
        release,
        npm: matches!(executable.as_str(), "npm" | "npx" | "pnpm" | "yarn"),
        cargo: executable == "cargo",
        pip: matches!(executable.as_str(), "pip" | "pip3"),
        docker: executable == "docker",
        network_unknown: false,
    }
}

fn command_facts_at_depth(command: &str, depth: usize) -> CommandFacts {
    let Some(commands) = shell_commands(command) else {
        return unknown_wrapper();
    };
    let mut facts = CommandFacts::default();
    for words in commands {
        facts.merge(facts_for_words(&words, depth));
    }
    facts
}

/// Read the executable words in each simple shell command. This intentionally
/// does not execute or expand the shell: it is a conservative inspection pass
/// used for the packet's safety boundary and report.
fn command_facts(command: &str) -> CommandFacts {
    command_facts_at_depth(command, 0)
}

fn infer_files(command: &str) -> Vec<&'static str> {
    let facts = command_facts(command);
    let mut files = Vec::new();
    if facts.npm {
        files.push("package.json");
    }
    if command_facts(command).npm && command.to_ascii_lowercase().contains(" ci") {
        files.push("package-lock.json");
    }
    if facts.cargo {
        files.push("Cargo.toml");
    }
    if facts.cargo && (command.contains("cargo build") || command.contains("cargo test")) {
        files.push("Cargo.lock");
    }
    if facts.pip && command.to_ascii_lowercase().contains("install") {
        files.push("requirements.txt");
    }
    if command.contains("go ") {
        files.push("go.mod");
    }
    files
}

fn infer_hosts(command: &str) -> BTreeSet<String> {
    let facts = command_facts(command);
    let mut hosts = BTreeSet::new();
    if facts.npm {
        hosts.insert("registry.npmjs.org".to_string());
    }
    if facts.cargo {
        hosts.insert("crates.io".to_string());
    }
    if facts.pip {
        hosts.insert("pypi.org".to_string());
    }
    if facts.docker {
        hosts.insert("container registry (set by image name)".to_string());
    }
    if facts.network_unknown {
        hosts.insert("Unknown (review wrapped or dynamic command)".to_string());
    }
    let url_re = Regex::new(r"https?://([A-Za-z0-9.-]+)").expect("valid host regex");
    for capture in url_re.captures_iter(command) {
        hosts.insert(capture[1].to_string());
    }
    hosts
}

fn provider_action_note(action: &str) -> String {
    if action.starts_with("actions/checkout@") {
        "actions/checkout is replaced by the mounted repository".to_string()
    } else if action.starts_with("actions/setup-") {
        format!("{action} is replaced by tools in the pinned image; verify versions")
    } else if action.starts_with("./") {
        format!("local action {action} is not executed; translate it to a shell step")
    } else {
        format!("provider action {action} is not executed; translate or replace it")
    }
}

fn redact_expressions(
    input: &str,
    secret_map: &mut BTreeMap<String, String>,
    warnings: &mut BTreeSet<String>,
) -> String {
    let secret_re = Regex::new(r"\$\{\{\s*secrets\.([A-Za-z_][A-Za-z0-9_]*)\s*\}\}").unwrap();
    let mut result = secret_re
        .replace_all(input, |caps: &regex::Captures<'_>| {
            let next = secret_map.len() + 1;
            let replacement = secret_map
                .entry(caps[1].to_string())
                .or_insert_with(|| format!("DRILL_SECRET_{next}"));
            format!("${replacement}")
        })
        .to_string();

    let expr_re = Regex::new(r"\$\{\{\s*([^}]+?)\s*\}\}").unwrap();
    if expr_re.is_match(&result) {
        warnings.insert(
            "GitHub context expressions remain; set matching environment values before execution"
                .into(),
        );
        result = expr_re
            .replace_all(&result, |caps: &regex::Captures<'_>| {
                let mut hash = Sha256::new();
                hash.update(caps[1].as_bytes());
                format!("$DRILL_CONTEXT_{}", &format!("{:x}", hash.finalize())[..8])
            })
            .to_string();
    }
    result
}

pub fn generate(options: &DrillOptions) -> Result<DrillReport> {
    if !is_pinned_image(&options.image) {
        return Err(DrillFailure::input(
            "image must include an immutable @sha256: digest with 64 hex characters",
        )
        .into());
    }
    if !options.workflow.is_file() {
        return Err(DrillFailure::input(format!(
            "workflow file was not found: {}",
            options.workflow.display()
        ))
        .into());
    }
    if !options.repo.is_dir() {
        return Err(DrillFailure::input(format!(
            "repository directory was not found: {}",
            options.repo.display()
        ))
        .into());
    }

    let source = fs::read_to_string(&options.workflow).map_err(|_| {
        DrillFailure::input(format!("could not read {}", options.workflow.display()))
    })?;
    let workflow: Workflow = serde_yaml::from_str(&source).map_err(|_| {
        DrillFailure::input(format!(
            "{} is not valid workflow YAML",
            options.workflow.display()
        ))
    })?;
    let available = workflow.jobs.keys().cloned().collect::<Vec<_>>().join(", ");
    let job = workflow.jobs.get(&options.job).ok_or_else(|| {
        DrillFailure::input(format!(
            "job '{}' was not found; available jobs: {available}",
            options.job
        ))
    })?;

    fs::create_dir_all(&options.out)
        .with_context(|| format!("could not create {}", options.out.display()))?;

    let mut secret_map = BTreeMap::new();
    let mut warnings = BTreeSet::new();
    let mut assumptions = Vec::new();
    let mut required_files = BTreeSet::new();
    let mut network_hosts = BTreeSet::new();
    let mut commands = Vec::new();
    let mut blocked = Vec::new();

    if job.runs_on.is_none() {
        warnings.insert("The selected job has no runs-on value".into());
    }
    if job.container.is_some() {
        assumptions.push(
            "The workflow defines a container; the requested pinned image takes precedence".into(),
        );
    }
    if job.needs.is_some() {
        warnings.insert(
            "Upstream job outputs are not reproduced; provide them as environment values".into(),
        );
    }

    for (key, value) in &job.env {
        if let Some(value) = yaml_string(value) {
            let value = redact_expressions(&value, &mut secret_map, &mut warnings);
            commands.push(format!("export {}={}", key, shell_env_value(&value)));
        }
    }

    for (index, step) in job.steps.iter().enumerate() {
        if let Some(action) = &step.uses {
            assumptions.push(provider_action_note(action));
        }
        let Some(run) = &step.run else { continue };
        let name = step
            .name
            .clone()
            .unwrap_or_else(|| format!("Step {}", index + 1));
        let mut command = redact_expressions(run, &mut secret_map, &mut warnings);
        for file in infer_files(&command) {
            required_files.insert(file.to_string());
        }
        network_hosts.extend(infer_hosts(&command));
        if let Some(dir) = &step.working_directory {
            required_files.insert(format!("{dir}/"));
            command = format!("cd {}\n{}", shell_quote(dir), command);
        }
        let mut exports = Vec::new();
        for (key, value) in &step.env {
            if let Some(value) = yaml_string(value) {
                let value = redact_expressions(&value, &mut secret_map, &mut warnings);
                exports.push(format!("export {}={}", key, shell_env_value(&value)));
            }
        }
        if !exports.is_empty() {
            command = format!("{}\n{}", exports.join("\n"), command);
        }
        if command_facts(&command).release && !options.allow_release {
            blocked.push(name);
        } else {
            commands.push(format!(
                "printf '\\n→ %s\\n' {}\n{}",
                shell_quote(&name),
                command
            ));
        }
    }

    let mut missing = Vec::new();
    for path in &required_files {
        let check = path.trim_end_matches('/');
        if !options.repo.join(check).exists() {
            missing.push(path.clone());
        }
    }
    if !missing.is_empty() {
        warnings.insert(format!("Missing required paths: {}", missing.join(", ")));
    }
    if commands.is_empty() {
        warnings.insert("No shell commands can be reproduced for this job".into());
    }

    let dockerfile = format!(
        "FROM {}\nWORKDIR /workspace\nCOPY run.sh /opt/cifail/run.sh\nRUN chmod +x /opt/cifail/run.sh\nENTRYPOINT [\"/opt/cifail/run.sh\"]\n",
        options.image
    );
    fs::write(options.out.join("Dockerfile"), dockerfile)?;

    let run_script = format!(
        "#!/bin/sh\nset -eu\ncd /workspace\n{}\nprintf '\\nFailover drill completed.\\n'\n",
        commands.join("\n\n")
    );
    fs::write(options.out.join("run.sh"), run_script)?;

    let mut env_example = String::from("# Anonymous inputs required by the selected job.\n");
    let mut generic_names = secret_map.values().cloned().collect::<Vec<_>>();
    generic_names.sort();
    for name in &generic_names {
        env_example.push_str(&format!("{name}=\n"));
    }
    fs::write(options.out.join(".env.example"), env_example)?;

    let ready = missing.is_empty() && !commands.is_empty();
    let mut report = DrillReport {
        schema_version: 1,
        job: options.job.clone(),
        image: options.image.clone(),
        ready,
        executed: false,
        execution_passed: None,
        commands_included: commands.len(),
        commands_blocked: blocked.len(),
        secret_inputs: secret_map.len(),
        required_files: required_files.into_iter().collect(),
        network_hosts: network_hosts.into_iter().collect(),
        provider_assumptions: assumptions,
        warnings: warnings.into_iter().collect(),
        packet_path: options.out.display().to_string(),
    };
    write_reports(&options.out, &report, &blocked)?;

    if options.execute {
        if !ready {
            return Err(DrillFailure::safety(
                "drill is not ready; fix the warnings in report.md before execution",
            )
            .into());
        }
        let docker = Command::new("docker")
            .arg("--version")
            .output()
            .map_err(|_| {
                DrillFailure::input(
                    "Docker is required for --execute; install Docker or omit --execute",
                )
            })?;
        if !docker.status.success() {
            return Err(DrillFailure::input(
                "Docker is not available; install Docker or omit --execute",
            )
            .into());
        }
        let tag = format!(
            "cifail-{}",
            Sha256::digest(options.job.as_bytes())[..4]
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>()
        );
        let build = Command::new("docker")
            .args(["build", "--quiet", "--tag", &tag])
            .arg(&options.out)
            .status()?;
        let passed = if build.success() {
            Command::new("docker")
                .args(["run", "--rm", "--env-file"])
                .arg(options.out.join(".env.example"))
                .arg("--volume")
                .arg(format!(
                    "{}:/workspace",
                    options.repo.canonicalize()?.display()
                ))
                .arg(&tag)
                .status()?
                .success()
        } else {
            false
        };
        report.executed = true;
        report.execution_passed = Some(passed);
        write_reports(&options.out, &report, &blocked)?;
        if !passed {
            return Err(DrillFailure::execution(
                "the container drill failed; review its command output and report.md",
            )
            .into());
        }
    }
    Ok(report)
}

fn write_reports(out: &Path, report: &DrillReport, blocked: &[String]) -> Result<()> {
    fs::write(
        out.join("drill.json"),
        serde_json::to_string_pretty(report)? + "\n",
    )?;
    let status = if report.execution_passed == Some(true) {
        "PASS"
    } else if report.ready {
        "READY"
    } else {
        "BLOCKED"
    };
    let bullet = |items: &[String]| {
        if items.is_empty() {
            "- None\n".to_string()
        } else {
            items.iter().map(|v| format!("- {v}\n")).collect()
        }
    };
    let markdown = format!(
        "# Failover drill: {}\n\n**Status: {status}**\n\n## Local drill\n\n- Container image: `{}`\n- Commands included: {}\n- Commands blocked: {}\n- Anonymous inputs: {}\n- Executed: {}\n\nRun from the repository root:\n\n```sh\ndocker build -t cifail-packet {}\ndocker run --rm --env-file {}/.env.example -v \"$PWD:/workspace\" cifail-packet\n```\n\n## Generic runner contract\n\nAny runner needs Docker, a checkout mounted at `/workspace`, and the anonymous inputs in `.env.example`.\n\n### Required paths\n{}\n### Network hosts\n{}\n### Provider assumptions\n{}\n### Blocked release steps\n{}\n### Warnings\n{}",
        report.job,
        report.image,
        report.commands_included,
        report.commands_blocked,
        report.secret_inputs,
        if report.executed { "yes" } else { "no" },
        report.packet_path,
        report.packet_path,
        bullet(&report.required_files),
        bullet(&report.network_hosts),
        bullet(&report.provider_assumptions),
        bullet(blocked),
        bullet(&report.warnings),
    );
    fs::write(out.join("report.md"), markdown)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_options(temp: &tempfile::TempDir) -> DrillOptions {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/sample-repo");
        DrillOptions {
            workflow: root.join(".github/workflows/release.yml"),
            job: "release-check".into(),
            image: "node:22-bookworm@sha256:8a34c4ab3ea2c5cd194f07e317b2a8f09461d3c8b05c4e34c8ccd56d56024c4d".into(),
            out: temp.path().join("packet"),
            repo: root,
            allow_release: false,
            execute: false,
        }
    }

    #[test]
    fn generates_safe_packet_and_redacts_secret_names() {
        let temp = tempfile::tempdir().unwrap();
        let report = generate(&sample_options(&temp)).unwrap();
        assert!(report.ready);
        assert_eq!(report.commands_blocked, 1);
        assert_eq!(report.secret_inputs, 1);
        let all = fs::read_to_string(temp.path().join("packet/run.sh")).unwrap()
            + &fs::read_to_string(temp.path().join("packet/report.md")).unwrap();
        assert!(!all.contains("NPM_TOKEN"));
        assert!(all.contains("DRILL_SECRET_1"));
        assert!(!all.contains("npm publish"));
    }

    #[test]
    fn rejects_unpinned_images() {
        let temp = tempfile::tempdir().unwrap();
        let mut options = sample_options(&temp);
        options.image = "node:22-bookworm".into();
        assert!(
            generate(&options)
                .unwrap_err()
                .to_string()
                .contains("immutable")
        );
    }

    #[test]
    fn release_command_requires_explicit_flag() {
        let temp = tempfile::tempdir().unwrap();
        let mut options = sample_options(&temp);
        options.allow_release = true;
        let report = generate(&options).unwrap();
        assert_eq!(report.commands_blocked, 0);
        assert!(
            fs::read_to_string(temp.path().join("packet/run.sh"))
                .unwrap()
                .contains("npm publish")
        );
    }

    #[test]
    fn recognizes_release_commands_through_options_wrappers_and_shells() {
        let cases = [
            "npm --access public publish",
            "npm pub",
            "pnpm publish",
            "yarn npm publish",
            "npm --access public \\\npublish",
            "git push --tags",
            "cargo publish",
            "docker push example/image",
            "gh release create v1",
            "env npm publish",
            "/usr/bin/env CI=1 npm --access public publish",
            "env -i TOKEN=value command -- npm pub",
            "command npm publish",
            "exec npm publish",
            "sudo -u runner npm publish",
            "sudo --preserve-env=TOKEN env TOKEN=value npm publish",
            "nohup npm publish",
            "nice -n 5 npm publish",
            "time -p npm publish",
            "timeout --signal=TERM 30 npm publish",
            "sh -c 'npm publish'",
            "bash -c \"env npm publish\"",
            "bash -lc \"env npm publish\"",
            "dash -ec 'npm publish'",
            "eval 'npm publish'",
        ];
        for command in cases {
            assert!(command_facts(command).release, "{command}");
        }
        for command in [
            "npm --access public publish",
            "env npm publish",
            "command -- npm publish",
            "exec env CI=1 npm publish",
            "sudo -u runner npm publish",
            "nohup npm publish",
            "nice -5 npm publish",
            "time npm publish",
            "timeout 30 npm publish",
            "sh -c 'npm publish'",
            "bash -lc 'npm publish'",
        ] {
            assert!(
                infer_hosts(command).contains("registry.npmjs.org"),
                "{command}"
            );
        }
    }

    #[test]
    fn wrapper_inspection_fails_closed_when_the_target_is_unclear() {
        for command in [
            "env --split-string='npm publish'",
            "env --unknown-option npm test",
            "command --unknown-option npm test",
            "exec -a",
            "sudo --unknown-option npm test",
            "sh -c",
            "eval",
            "'unfinished",
        ] {
            assert!(command_facts(command).release, "{command}");
            assert!(
                infer_hosts(command).contains("Unknown (review wrapped or dynamic command)"),
                "{command}"
            );
        }
        assert!(!command_facts("env CI=1 npm test").release);
        assert!(command_facts("env CI=1 npm test").npm);
        assert!(command_facts("$RELEASE_TOOL publish").release);
    }
}

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
            "{state}  {}\npacket  {}\nsteps   {} included, {} blocked\ninputs  {} anonymous secret input(s)\n",
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

fn is_release_command(command: &str) -> bool {
    let lower = command.to_ascii_lowercase();
    [
        "npm publish",
        "cargo publish",
        "docker push",
        "gh release create",
        "twine upload",
        "gem push",
        "helm push",
        "kubectl apply",
        "terraform apply",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

fn infer_files(command: &str) -> Vec<&'static str> {
    let mut files = Vec::new();
    if command.contains("npm ") || command.contains("npx ") {
        files.push("package.json");
    }
    if command.contains("npm ci") {
        files.push("package-lock.json");
    }
    if command.contains("cargo ") {
        files.push("Cargo.toml");
    }
    if command.contains("cargo build") || command.contains("cargo test") {
        files.push("Cargo.lock");
    }
    if command.contains("pip install") {
        files.push("requirements.txt");
    }
    if command.contains("go ") {
        files.push("go.mod");
    }
    files
}

fn infer_hosts(command: &str) -> BTreeSet<String> {
    let lower = command.to_ascii_lowercase();
    let mut hosts = BTreeSet::new();
    if lower.contains("npm ci") || lower.contains("npm install") || lower.contains("npm publish") {
        hosts.insert("registry.npmjs.org".to_string());
    }
    if lower.contains("cargo ") {
        hosts.insert("crates.io".to_string());
    }
    if lower.contains("pip install") {
        hosts.insert("pypi.org".to_string());
    }
    if lower.contains("docker ") {
        hosts.insert("container registry (set by image name)".to_string());
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
        if is_release_command(&command) && !options.allow_release {
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
        "# Failover drill: {}\n\n**Status: {status}**\n\n## Local drill\n\n- Container image: `{}`\n- Commands included: {}\n- Commands blocked: {}\n- Anonymous secret inputs: {}\n- Executed: {}\n\nRun from the repository root:\n\n```sh\ndocker build -t cifail-packet {}\ndocker run --rm --env-file {}/.env.example -v \"$PWD:/workspace\" cifail-packet\n```\n\n## Generic runner contract\n\nAny runner needs Docker, a checkout mounted at `/workspace`, and the anonymous inputs in `.env.example`.\n\n### Required paths\n{}\n### Network hosts\n{}\n### Provider assumptions\n{}\n### Blocked release steps\n{}\n### Warnings\n{}",
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
}

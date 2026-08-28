use anyhow::Result;
use cifail::{DrillOptions, failure_exit_code, generate};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};

const SAMPLE_IMAGE: &str =
    "node:22-bookworm@sha256:8a34c4ab3ea2c5cd194f07e317b2a8f09461d3c8b05c4e34c8ccd56d56024c4d";

#[derive(Parser)]
#[command(name = "cifail", version, about = "Prove one GitHub Actions job can run on another runner.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Turn one workflow job into a portable drill packet
    Drill {
        /// GitHub Actions workflow YAML file
        #[arg(long)]
        workflow: PathBuf,
        /// Job key inside the workflow
        #[arg(long)]
        job: String,
        /// Container image pinned with @sha256:<digest>
        #[arg(long)]
        image: String,
        /// Directory for the generated packet
        #[arg(long, default_value = ".ci-failover")]
        out: PathBuf,
        /// Repository root mounted into the runner
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        /// Run the generated packet with Docker
        #[arg(long)]
        execute: bool,
        /// Include release and publish commands; use only with test targets
        #[arg(long)]
        allow_release: bool,
        /// Print the report as JSON
        #[arg(long)]
        json: bool,
    },
    /// Run the bundled sample in a temporary sandbox
    Demo {
        /// Print the report as JSON
        #[arg(long)]
        json: bool,
        /// Keep the sample at this path instead of a temporary path
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

fn copy_tree(source: &Path, target: &Path) -> Result<()> {
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let destination = target.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &destination)?;
        } else {
            fs::copy(entry.path(), destination)?;
        }
    }
    Ok(())
}

fn print_report(report: &cifail::DrillReport, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(report)?);
    } else {
        print!("{}", report.summary());
        if report.commands_blocked > 0 {
            println!("safety   release commands omitted; inspect report.md");
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    match Cli::parse().command {
        Commands::Drill {
            workflow,
            job,
            image,
            out,
            repo,
            execute,
            allow_release,
            json,
        } => {
            let report = generate(&DrillOptions {
                workflow,
                job,
                image,
                out,
                repo,
                execute,
                allow_release,
            })?;
            print_report(&report, json)
        }
        Commands::Demo { json, out } => {
            let (sandbox, _guard) = if let Some(path) = out {
                (path, None)
            } else {
                let temp = tempfile::Builder::new().prefix("cifail-demo-").tempdir()?;
                let path = temp.keep();
                (path, None::<tempfile::TempDir>)
            };
            let sample = sandbox.join("sample-repo");
            copy_tree(
                &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/sample-repo"),
                &sample,
            )?;
            let report = generate(&DrillOptions {
                workflow: sample.join(".github/workflows/release.yml"),
                job: "release-check".into(),
                image: SAMPLE_IMAGE.into(),
                out: sandbox.join("failover-packet"),
                repo: sample,
                execute: false,
                allow_release: false,
            })?;
            print_report(&report, json)?;
            if !json {
                println!("demo     sample data only; nothing was saved to your repository");
            }
            Ok(())
        }
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("cifail: {error:#}");
        std::process::exit(failure_exit_code(&error));
    }
}

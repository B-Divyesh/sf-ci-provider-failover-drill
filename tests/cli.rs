use assert_cmd::Command;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

const IMAGE: &str =
    "node:22-bookworm@sha256:8a34c4ab3ea2c5cd194f07e317b2a8f09461d3c8b05c4e34c8ccd56d56024c4d";

fn sample() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/sample-repo")
}

#[test]
fn demo_json_is_clean_for_scripts() {
    let temp = tempfile::tempdir().unwrap();
    let output = Command::cargo_bin("cifail")
        .unwrap()
        .args(["demo", "--json", "--out"])
        .arg(temp.path())
        .output()
        .unwrap();
    assert!(output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["job"], "release-check");
    assert_eq!(value["ready"], true);
}

#[test]
fn documented_exit_codes_are_structural() {
    let missing_job = Command::cargo_bin("cifail")
        .unwrap()
        .args(["drill", "--workflow"])
        .arg(sample().join(".github/workflows/release.yml"))
        .args(["--job", "not-a-job", "--image", IMAGE, "--repo"])
        .arg(sample())
        .assert();
    missing_job.code(2);

    let temp = tempfile::tempdir().unwrap();
    let incomplete = temp.path().join("incomplete-repo");
    copy_tree(&sample(), &incomplete);
    fs::remove_file(incomplete.join("package.json")).unwrap();
    Command::cargo_bin("cifail")
        .unwrap()
        .args(["drill", "--workflow"])
        .arg(incomplete.join(".github/workflows/release.yml"))
        .args(["--job", "release-check", "--image", IMAGE, "--repo"])
        .arg(&incomplete)
        .args(["--out"])
        .arg(temp.path().join("safety-packet"))
        .arg("--execute")
        .assert()
        .code(3);

    let fake_bin = temp.path().join("fake-bin");
    fs::create_dir_all(&fake_bin).unwrap();
    let docker = fake_bin.join("docker");
    fs::write(
        &docker,
        "#!/bin/sh\n[ \"$1\" = \"--version\" ] && exit 0\nexit 1\n",
    )
    .unwrap();
    fs::set_permissions(&docker, fs::Permissions::from_mode(0o755)).unwrap();
    let path = format!("{}:{}", fake_bin.display(), std::env::var("PATH").unwrap());
    Command::cargo_bin("cifail")
        .unwrap()
        .env("PATH", path)
        .args(["drill", "--workflow"])
        .arg(sample().join(".github/workflows/release.yml"))
        .args(["--job", "release-check", "--image", IMAGE, "--repo"])
        .arg(sample())
        .args(["--out"])
        .arg(temp.path().join("execution-packet"))
        .arg("--execute")
        .assert()
        .code(4);
}

fn copy_tree(source: &Path, target: &Path) {
    fs::create_dir_all(target).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let destination = target.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &destination);
        } else {
            fs::copy(entry.path(), destination).unwrap();
        }
    }
}

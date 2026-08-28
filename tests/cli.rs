use assert_cmd::Command;

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

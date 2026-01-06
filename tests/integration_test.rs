use std::process::Command;

#[test]
fn integration_test_basic_analysis() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "mcl1-analyzer", "data/sample_interactions.csv"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn integration_test_pathway_prediction() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "pathway-predictor",
            "--",
            "-i",
            "data/mcl1_data.json",
            "-o",
            "results/predictions.txt"
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn integration_test_interactive_mode() {
    // Test that the interactive analyzer binary compiles and runs without panicking
    let output = Command::new("cargo")
        .args(&["run", "--bin", "interactive-analyzer"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}
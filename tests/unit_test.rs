use mcl1_regulator::{MCL1Analyzer, PathwayPredictor};
use std::fs;

#[test]
fn test_mcl1_analyzer_initialization() {
    let analyzer = MCL1Analyzer::new();
    assert!(analyzer != None);
}

#[test]
fn test_pathway_predictor_initialization() {
    let predictor = PathwayPredictor::new();
    assert!(predictor != None);
}

#[test]
fn test_sample_data_loading() {
    let sample_data = fs::read_to_string("data/sample_interactions.csv");
    assert!(sample_data.is_ok());
}

#[test]
fn test_metabolic_pathway_prediction() {
    let analyzer = MCL1Analyzer::new();
    let result = analyzer.predict_metabolic_pathways("data/sample_interactions.csv");
    assert!(result.is_ok());
}
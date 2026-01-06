rust
//! MCL1-Regulator: A Rust-based computational tool for analyzing MCL1 protein interactions
//! and predicting cancer cell survival pathways.

/// Main module for MCL1 analysis functionality
pub mod mcl1_analyzer;
/// Module for metabolic pathway prediction
pub mod pathway_predictor;

/// Re-export key types and functions for easy access
pub use mcl1_analyzer::MCL1Analyzer;
pub use pathway_predictor::PathwayPredictor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = MCL1Analyzer::new();
        assert!(analyzer.is_some());
    }

    #[test]
    fn test_predictor_creation() {
        let predictor = PathwayPredictor::new();
        assert!(predictor.is_some());
    }
}
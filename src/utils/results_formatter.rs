rust
//! Utility module for formatting analysis results
//!
//! Provides functions to format and serialize MCL1 analysis results
//! into various output formats for reporting and visualization.

use std::collections::HashMap;
use serde_json::Value;

/// Formats MCL1 interaction data into a human-readable string
pub fn format_interactions(interactions: &[(String, String, f64)]) -> String {
    let mut output = String::new();
    output.push_str("MCL1 Interactions:\n");
    output.push_str("==================\n");
    
    for (protein_a, protein_b, score) in interactions {
        output.push_str(&format!(
            "{} <-> {} (score: {:.3})\n",
            protein_a, protein_b, score
        ));
    }
    
    output
}

/// Formats pathway prediction results into structured JSON
pub fn format_pathway_predictions(predictions: &HashMap<String, f64>) -> String {
    let mut json_map = serde_json::Map::new();
    
    for (pathway, score) in predictions {
        json_map.insert(
            pathway.clone(),
            Value::Number(serde_json::Number::from_f64(*score).unwrap_or(0.0))
        );
    }
    
    serde_json::to_string_pretty(&Value::Object(json_map)).unwrap_or_else(|_| "{}".to_string())
}

/// Formats analysis summary into a concise report
pub fn format_summary(
    total_interactions: usize,
    active_pathways: usize,
    avg_score: f64,
) -> String {
    format!(
        "Analysis Summary:\n\
         ----------------\n\
         Total Interactions: {}\n\
         Active Pathways: {}\n\
         Average Score: {:.3}\n",
        total_interactions, active_pathways, avg_score
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_interactions() {
        let interactions = vec![
            ("MCL1".to_string(), "BAX".to_string(), 0.85),
            ("MCL1".to_string(), "BCL2".to_string(), 0.72),
        ];
        
        let result = format_interactions(&interactions);
        assert!(result.contains("MCL1 <-> BAX"));
        assert!(result.contains("MCL1 <-> BCL2"));
    }

    #[test]
    fn test_format_pathway_predictions() {
        let mut predictions = HashMap::new();
        predictions.insert("mTOR_pathway".to_string(), 0.95);
        predictions.insert("glycolysis".to_string(), 0.67);
        
        let result = format_pathway_predictions(&predictions);
        assert!(result.contains("mTOR_pathway"));
        assert!(result.contains("glycolysis"));
    }

    #[test]
    fn test_format_summary() {
        let result = format_summary(150, 23, 0.785);
        assert!(result.contains("Total Interactions: 150"));
        assert!(result.contains("Active Pathways: 23"));
        assert!(result.contains("Average Score: 0.785"));
    }
}
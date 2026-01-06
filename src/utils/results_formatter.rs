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

/// Formats pathway prediction results into JSON structure
pub fn format_pathway_predictions(
    predictions: &HashMap<String, Vec<(String, f64)>>,
) -> Value {
    let mut result = HashMap::new();
    
    for (pathway, scores) in predictions {
        let mut pathway_data = Vec::new();
        
        for (protein, score) in scores {
            pathway_data.push(serde_json::json!({
                "protein": protein,
                "score": score
            }));
        }
        
        result.insert(pathway.clone(), pathway_data);
    }
    
    serde_json::json!(result)
}

/// Formats analysis summary into a structured report
pub fn format_summary(
    total_interactions: usize,
    active_pathways: usize,
    predicted_drugs: usize,
) -> String {
    format!(
        r#"Analysis Summary:
=================
Total Interactions: {}
Active Pathways: {}
Predicted Drug Targets: {}

This analysis provides insights into MCL1's dual role in cell survival 
and metabolic regulation, supporting the development of targeted cancer 
therapies with reduced cardiotoxicity.
"#,
        total_interactions, active_pathways, predicted_drugs
    )
}
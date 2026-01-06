use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicPathway {
    pub id: String,
    pub name: String,
    pub activation_score: f64,
    pub associated_proteins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathwayPredictionResult {
    pub predicted_pathways: Vec<MetabolicPathway>,
    pub confidence_score: f64,
    pub prediction_timestamp: String,
}

pub struct PathwayPredictor {
    pub model_parameters: HashMap<String, f64>,
}

impl PathwayPredictor {
    pub fn new() -> Self {
        let mut params = HashMap::new();
        params.insert("mcl1_threshold".to_string(), 0.7);
        params.insert("metabolism_weight".to_string(), 0.8);
        params.insert("interaction_weight".to_string(), 0.6);
        
        PathwayPredictor {
            model_parameters: params,
        }
    }

    pub fn predict_pathways(
        &self,
        mcl1_interactions: &[String],
        metabolic_data: &HashMap<String, f64>,
    ) -> PathwayPredictionResult {
        let mut pathways = Vec::new();
        let mut total_confidence = 0.0;
        
        // Simulate pathway prediction based on input data
        for (i, protein) in mcl1_interactions.iter().enumerate() {
            let score = self.calculate_activation_score(protein, metabolic_data);
            total_confidence += score;
            
            pathways.push(MetabolicPathway {
                id: format!("pathway_{}", i),
                name: format!("Metabolic Pathway {}", i + 1),
                activation_score: score,
                associated_proteins: vec![protein.clone()],
            });
        }
        
        let avg_confidence = if !pathways.is_empty() {
            total_confidence / pathways.len() as f64
        } else {
            0.0
        };
        
        PathwayPredictionResult {
            predicted_pathways: pathways,
            confidence_score: avg_confidence,
            prediction_timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    fn calculate_activation_score(&self, protein: &str, metabolic_data: &HashMap<String, f64>) -> f64 {
        let base_score = match protein.as_str() {
            "mcl1" => 0.9,
            "mtor" => 0.85,
            "akt" => 0.75,
            _ => 0.5,
        };
        
        let metabolic_contribution = metabolic_data.get(protein)
            .copied()
            .unwrap_or(0.0)
            * self.model_parameters.get("metabolism_weight").copied().unwrap_or(0.8);
            
        let interaction_contribution = self.model_parameters.get("interaction_weight").copied().unwrap_or(0.6);
        
        (base_score + metabolic_contribution + interaction_contribution) / 3.0
    }
}
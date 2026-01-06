use crate::models::{ProteinInteraction, DrugTarget};
use crate::utils::math::calculate_cardiotoxicity_score;

#[derive(Debug, Clone)]
pub struct CardiotoxicityPredictor {
    pub threshold: f64,
}

impl CardiotoxicityPredictor {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    pub fn predict_cardiotoxicity(&self, interactions: &[ProteinInteraction]) -> Vec<DrugTarget> {
        interactions
            .iter()
            .filter_map(|interaction| {
                let score = calculate_cardiotoxicity_score(interaction);
                if score >= self.threshold {
                    Some(DrugTarget {
                        target_id: interaction.target.clone(),
                        cardiotoxicity_score: score,
                        confidence: 0.95,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}
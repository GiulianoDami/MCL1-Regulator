use crate::models::MCL1Interaction;
use crate::utils::calculate_cardiotoxicity_risk;

pub struct CardiotoxicityPredictor {}

impl CardiotoxicityPredictor {
    pub fn new() -> Self {
        CardiotoxicityPredictor {}
    }

    pub fn predict_cardiotoxicity(&self, interactions: &[MCL1Interaction]) -> f64 {
        let total_risk: f64 = interactions
            .iter()
            .map(|interaction| calculate_cardiotoxicity_risk(interaction))
            .sum();

        total_risk / interactions.len() as f64
    }
}
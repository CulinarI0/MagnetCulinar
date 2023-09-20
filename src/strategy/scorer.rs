use crate::models::tag::Tag;

pub trait ScoringStrategy {
    fn calculate_score(&self, tags: &[Tag]) -> f64;
}

pub struct DefaultScoringStrategy;

impl ScoringStrategy for DefaultScoringStrategy {
    fn calculate_score(&self, tags: &[Tag]) -> f64 {
        let mut score = 0.0;

        for tag in tags {

            // TODO remove these constants to things coming from a database
            match tag.name.as_str() {
                "dry" => score += 1.0,
                "fatty" => score += 1.0,
                "lamb" => score += 1.0,
                _ => {} 
            }
        }

        score
    }
}


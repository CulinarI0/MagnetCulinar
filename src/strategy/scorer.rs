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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tags() {
        let strategy = DefaultScoringStrategy;
        let tags: Vec<Tag> = vec![];
        let score = strategy.calculate_score(&tags);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_single_tag() {
        let strategy = DefaultScoringStrategy;
        let tags = vec![Tag::new("dry", 1.0)];
        let score = strategy.calculate_score(&tags);
        assert_eq!(score, 1.0);
    }

    #[test]
    fn test_multiple_tags() {
        let strategy = DefaultScoringStrategy;
        let tags = vec![
            Tag::new("dry", 1.0),
            Tag::new("fatty", 1.0),
            Tag::new("lamb", 1.0),
        ];
        let score = strategy.calculate_score(&tags);
        assert_eq!(score, 3.0);
    }
}

#[path = "../src/models/mod.rs"] mod models;
#[path = "../src/strategy/mod.rs"] mod strategy;
// TODO check if unit tests will stay here
#[cfg(test)]
mod tests {
    use crate::{strategy::scorer::{DefaultScoringStrategy, ScoringStrategy}, models::tag::Tag};

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

use crate::models::tag::Tag;

pub trait MatcherStrategy {
    fn match_score(&self, input_tags: &[Tag], target_tags: &[Tag]);
}

pub struct DefaultMatcherStrategy;

impl MatcherStrategy for DefaultMatcherStrategy {
    fn match_score(&self, input_tags: &[Tag], target_tags: &[Tag])  {  
    }

}

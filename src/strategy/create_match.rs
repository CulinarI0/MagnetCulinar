use crate::models::tag::Tag;

pub trait MatcherStrategy {
    fn calculate_score(&self, input_tags: &[Tag], target_tags: &[Tag]);
}

pub struct DefaultMatcherStrategy;
fn create_nodes(){
    
}
fn create_tree(){
    create_nodes();
}
impl MatcherStrategy for DefaultMatcherStrategy {
    fn calculate_score(&self, input_tags: &[Tag], target_tags: &[Tag])  {  
    }

}

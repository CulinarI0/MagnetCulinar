mod models;
mod strategy;
use models::tag::Tag;
use models::info::Info;

use crate::strategy::scorer::{ScoringStrategy, DefaultScoringStrategy};
fn main() {
    let tag1 = Tag::new("dry", 0.8);
    let tag2 = Tag::new("fatty", 0.6);
    let tag3 = Tag::new("lamb meat", 0.9);
    let tag4 = Tag::new("fish meat", 0.9);
    let tag5 = Tag::new("cow meat", 0.9);
    let _tag6 = Tag::new("capybara meat", 0.9);
    let _tag7 = Tag::new("deer meat", 0.9);
    
    //Infos would come from a specifc DB, and the most important one is the tags
    let info1 = Info::new("Cow meat", vec![tag1, tag2, tag5]);
    let info2 = Info::new("Fish meat", vec![tag3, tag4]);

    info1.display();
    println!("---");
    info2.display();
    let strategy = DefaultScoringStrategy;

    println!("Info cow meat Score: {}", strategy.calculate_score(&info1.tags));
    println!("Info fish meat Score: {}", strategy.calculate_score(&info2.tags));
}

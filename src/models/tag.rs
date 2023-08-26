pub struct Tag {
    pub name: String,
    // I think it will come from DB
    // TODO create importance representation
    pub score: f64,
}

impl Tag {
    pub fn new(name: &str, score: f64) -> Tag {
        Tag {
            name: name.to_string(),
            score,
        }
    } 
}

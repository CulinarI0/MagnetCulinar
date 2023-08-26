use super::tag::Tag;

pub struct Info {
    // data will be anohter struct
    pub data: String,
    pub tags: Vec<Tag>,
}

impl Info {
    pub fn new(data: &str, tags: Vec<Tag>) -> Info {
        Info {
            data: data.to_string(),
            tags,
        }
    }
    pub fn display(&self) {
        println!("Data: {}", self.data);
        println!("Tags:");
        for tag in &self.tags {
            println!("  Name: {}, Score: {}", tag.name, tag.score);
        }
    }
}

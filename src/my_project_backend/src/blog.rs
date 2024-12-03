use candid::CandidType;

#[derive(Clone, CandidType)]
pub struct Blog {
    title: String,
    date: u64,
    content: String,
    tags: Vec<String>,
}

impl Blog {
    pub fn new(title: String, content: String, tags: Vec<String>) -> Self {
        Self {
            title,
            date: ic_cdk::api::time(),
            content,
            tags,
        }
    }
}
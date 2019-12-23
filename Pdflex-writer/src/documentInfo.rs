pub struct DocumentInfo {
    pub title: String,
    pub author: String,
    pub subject: String,
    pub keywords: String
}

impl DocumentInfo {
    pub fn new() -> DocumentInfo {
        DocumentInfo { 
            title: "".to_string(), 
            author: "".to_string(), 
            subject: "".to_string(), 
            keywords: "".to_string()
        }
    }
}
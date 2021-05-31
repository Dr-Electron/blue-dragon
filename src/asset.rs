use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    #[serde(rename = "ID")]
    id: String,
    name: String,
    symbol: String,
    supply: i32,
    #[serde(rename = "transactionID")]
    transaction_id: String,
}

impl Asset {
    pub fn contains_bad_word(&self, bad_words: &str) -> bool {
        search(&self.name, &bad_words).len() > 0 ||
        search(&self.id, &bad_words).len() > 0 ||
        search(&self.symbol, &bad_words).len() > 0
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }
    
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

fn search<'a>(
    query: &'a str,
    contents: &str,
) -> Vec<&'a str> {
    let _query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        if _query.contains(&line.to_lowercase()) {
            results.push(query);
        }
    }

    results
}
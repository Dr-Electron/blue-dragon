use regex::Regex;
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

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
        search(&self.name, &bad_words).len() > 0
            || search(&self.id, &bad_words).len() > 0
            || search(&self.symbol, &bad_words).len() > 0
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

fn search<'a>(query: &'a str, contents: &str) -> Vec<&'a str> {
    let mut _query = query
        .to_lowercase()
        //Normalize string
        .nfd()
        .filter(char::is_ascii)
        .collect::<String>()
        //Turn tabs into spaces
        .replace("\t", " ")
        //Get rid of zero-width spaces
        .replace("\u{200b}", "");

    //Convert multiple re-occurring whitespaces into a single space
    let re = Regex::new(r"^[\s\p{Zs}]+|[\s\p{Zs}]+$").unwrap();
    _query = re.replace_all(&_query, "").to_string();
    let re = Regex::new(r"[\s\p{Zs}]{2,}").unwrap();
    _query = re.replace_all(&_query, "").to_string();

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        if _query.contains(&line.to_lowercase()) {
            results.push(query);
        } else if _query.replace(" ", "").contains(&line.to_lowercase()) {
            results.push(query);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let query = "tester";
        let contents = "\
Word1
test
three.";

        assert_eq!(vec![query], search(query, contents));
    }

    #[test]
    fn normalize_test() {
        let query = "tèst";
        let contents = "\
Word1
test
three.";

        assert_eq!(vec![query], search(query, contents));
    }

    #[test]
    fn spaced_tab_test() {
        let query = "te\tster";
        let contents = "\
Word1
te st
three.";

        assert_eq!(vec![query], search(query, contents));
    }

    #[test]
    fn zero_width_stripping_test() {
        let query = "\u{200b}tester";
        let contents = "\
Word1
test
three.";

        assert_eq!(vec![query], search(query, contents));
    }

    #[test]
    fn multi_whitespace_stripping_test() {
        let query = "t    e s    t  e   r";
        let contents = "\
Word1
te st
three.";

        assert_eq!(vec![query], search(query, contents));
    }

    #[test]
    fn remove_whitespaces_test() {
        let query = "t e s t e r";
        let contents = "\
Word1
test
three.";

        assert_eq!(vec![query], search(query, contents));
    }
}

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Ord, PartialOrd)]
pub struct CaseInsensitiveString {
    pub inner: String,
}

impl CaseInsensitiveString {
    pub fn new(s: &str) -> Self {
        Self {
            inner: s.to_string(),
        }
    }
}

impl Eq for CaseInsensitiveString {}
impl PartialEq for CaseInsensitiveString {
    fn eq(self: &'_ Self, other: &'_ Self) -> bool {
        self.inner.to_lowercase() == other.inner.to_lowercase()
    }
}

impl Hash for CaseInsensitiveString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.to_lowercase().hash(state);
    }
}

/// Removes duplicates from a vector in a case-insensitive manner. Order is not retained.
pub fn deduplicate_case_insensitive(vec: Vec<CaseInsensitiveString>) -> Vec<CaseInsensitiveString> {
    let set: HashSet<CaseInsensitiveString> = vec.into_iter().collect();
    set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::core::case_insensitive_string::{
        deduplicate_case_insensitive, CaseInsensitiveString,
    };

    #[test]
    fn removes_duplicates() {
        let input = vec![
            CaseInsensitiveString::new("bar"),
            CaseInsensitiveString::new("baz"),
            CaseInsensitiveString::new("foo"),
            CaseInsensitiveString::new("Bar"),
            CaseInsensitiveString::new("Baz"),
        ];
        let expected = vec!["bar", "baz", "foo"];

        let results: Vec<String> = deduplicate_case_insensitive(input)
            .into_iter()
            .map(|s| s.inner)
            .collect();

        assert_eq!(results.len(), expected.len());
        assert_eq!(results.contains(&String::from("bar")), true);
        assert_eq!(results.contains(&String::from("baz")), true);
        assert_eq!(results.contains(&String::from("foo")), true);
        assert_eq!(results.contains(&String::from("Bar")), false);
        assert_eq!(results.contains(&String::from("Baz")), false);
    }

    /// Retains the first seen value - in this case "Bar", not "bar"
    #[test]
    fn keeps_first_seen() {
        let input = vec![
            CaseInsensitiveString::new("Bar"),
            CaseInsensitiveString::new("baz"),
            CaseInsensitiveString::new("foo"),
            CaseInsensitiveString::new("bar"),
            CaseInsensitiveString::new("Baz"),
        ];
        let expected = vec!["bar", "baz", "foo"];

        let results: Vec<String> = deduplicate_case_insensitive(input)
            .into_iter()
            .map(|s| s.inner)
            .collect();

        assert_eq!(results.len(), expected.len());
        assert_eq!(results.contains(&String::from("Bar")), true);
        assert_eq!(results.contains(&String::from("baz")), true);
        assert_eq!(results.contains(&String::from("foo")), true);
        assert_eq!(results.contains(&String::from("bar")), false);
        assert_eq!(results.contains(&String::from("Baz")), false);
    }
}

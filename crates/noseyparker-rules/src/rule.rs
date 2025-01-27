use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A pattern-based rule as represented syntactically.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Rule {
    /// The human-readable name of the rule
    pub name: String,

    /// A globally-unique identifier for the rule
    pub id: String,

    /// The regex pattern that the rule uses
    pub pattern: String,

    /// Example inputs that this rule is expected to match
    #[serde(default)]
    pub examples: Vec<String>,

    /// Example inputs that this rule is expected _not_ to match
    #[serde(default)]
    pub negative_examples: Vec<String>,

    /// Freeform references for the rule; usually URLs
    #[serde(default)]
    pub references: Vec<String>,
}

lazy_static! {
    // used to strip out vectorscan-style comments like `(?# this is a comment)`,
    // which Rust's regex crate doesn't like
    static ref RULE_COMMENTS_PATTERN: Regex = Regex::new(r"\(\?#[^)]*\)")
        .expect("comment-stripping regex should compile");
}

impl Rule {
    /// Get the pattern for this rule with any comments removed.
    pub fn uncommented_pattern(&self) -> Cow<'_, str> {
        RULE_COMMENTS_PATTERN.replace_all(&self.pattern, "")
    }

    // NOTE: Some of the patterns from default rules are complicated patterns that require more
    // than the default regex size limit to compile. 16MiB has been enough so far...
    const REGEX_SIZE_LIMIT: usize = 16 * 1024 * 1024;

    fn build_regex(pattern: &str) -> Result<regex::bytes::Regex> {
        let pattern = regex::bytes::RegexBuilder::new(pattern)
            .unicode(false)
            .size_limit(Self::REGEX_SIZE_LIMIT)
            .build()?;
        Ok(pattern)
    }

    /// Compile this pattern into a regular expression.
    pub fn as_regex(&self) -> Result<regex::bytes::Regex> {
        Self::build_regex(&self.uncommented_pattern())
    }

    /// Compile this rule into a regex with an end-of-line anchor appended.
    /// This will ensure that any matches of this rule occur at the end of input.
    ///
    /// Examples:
    ///
    /// ```
    /// # use pretty_assertions::assert_eq;
    /// # use noseyparker_rules::Rule;
    /// let r = Rule {
    ///     name: "Test rule".to_string(),
    ///     id: "test.1".to_string(),
    ///     pattern: r"hello\s*world".to_string(),
    ///     examples: vec![],
    ///     negative_examples: vec![],
    ///     references: vec![],
    /// };
    /// assert_eq!(r.as_anchored_regex().unwrap().as_str(), r"hello\s*world$");
    /// ```
    pub fn as_anchored_regex(&self) -> Result<regex::bytes::Regex> {
        Self::build_regex(&format!("{}$", self.uncommented_pattern()))
    }
}

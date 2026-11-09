//! Request validation with the same messages the frontend already shows.

/// Collects every failed rule so the client gets all problems at once,
/// joined with ", " like before.
#[derive(Default)]
pub struct Validator {
    errors: Vec<String>,
}

pub fn is_blank(v: Option<&str>) -> bool {
    v.is_none_or(|s| s.trim().is_empty())
}

/// Lenient structural check (like Hibernate's `@Email`): one `@`, non-empty
/// local part and domain, no whitespace.
pub fn is_email(s: &str) -> bool {
    let s = s.trim();
    let Some((local, domain)) = s.split_once('@') else {
        return false;
    };
    !local.is_empty()
        && !domain.is_empty()
        && !domain.contains('@')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
        && !s.chars().any(char::is_whitespace)
}

impl Validator {
    pub fn check(&mut self, ok: bool, message: &str) -> &mut Self {
        if !ok {
            self.errors.push(message.to_owned());
        }
        self
    }

    pub fn required(&mut self, v: Option<&str>, message: &str) -> &mut Self {
        self.check(!is_blank(v), message)
    }

    pub fn max_chars(&mut self, v: Option<&str>, max: usize, message: &str) -> &mut Self {
        self.check(v.is_none_or(|s| s.chars().count() <= max), message)
    }

    pub fn min_chars(&mut self, v: Option<&str>, min: usize, message: &str) -> &mut Self {
        // Absent values are reported by `required`, not here.
        self.check(v.is_none_or(|s| s.chars().count() >= min), message)
    }

    pub fn email(&mut self, v: Option<&str>, message: &str) -> &mut Self {
        // Blank values are reported by `required`.
        self.check(is_blank(v) || v.is_some_and(is_email), message)
    }

    pub fn finish(&mut self) -> Result<(), String> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.join(", "))
        }
    }
}

/// Implemented by request bodies; called by the `ValidJson` extractor.
pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_rules() {
        assert!(is_email("a@b.co"));
        assert!(is_email("a@localhost"));
        assert!(!is_email("ab.co"));
        assert!(!is_email("a b@c.d"));
        assert!(!is_email("a@b@c"));
        assert!(!is_email("@b.co"));
    }

    #[test]
    fn collects_all_errors() {
        let err = Validator::default()
            .required(None, "Name is required")
            .email(Some("nope"), "Must be a valid email")
            .max_chars(Some("abc"), 2, "Too long")
            .finish()
            .unwrap_err();
        assert_eq!(err, "Name is required, Must be a valid email, Too long");
    }
}
